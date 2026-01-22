use crate::{
    controllers::products_controller::SearchQuery,
    models::{dto::ProductSearchDocument, error::AppError},
};
use meilisearch_sdk::{
    client::Client,
    search::SearchResults,
    settings::{self, Settings},
    tasks::Task,
};
use uuid::Uuid;

#[derive(Clone)]
pub struct SearchService {
    client: Client,
}

impl SearchService {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn add_documents(&self, products: &[ProductSearchDocument]) -> Result<(), AppError> {
        let index = self.client.index(ProductSearchDocument::INDEX_NAME);

        // ส่งข้อมูลเป็น Batch
        index
            .add_documents(products, Some("id"))
            .await
            .map_err(|e| {
                println!("Meilisearch Error: {:?}", e); // log error เผื่อไว้ debug
                AppError::InternalServerError("Failed to batch index products".into())
            })?;

        Ok(())
    }

    // ฟังก์ชันเพิ่มสินค้าลง Index (ใช้ตอน Create/Update Product)
    pub async fn add_product(&self, product: ProductSearchDocument) -> Result<(), AppError> {
        let index = self.client.index(ProductSearchDocument::INDEX_NAME);

        // Meilisearch รับเป็น Array ของ documents
        index
            .add_documents(&[product], Some("id"))
            .await
            .map_err(|_| AppError::InternalServerError("Failed to index product".into()))?;

        Ok(())
    }

    pub async fn delete_product(&self, id: Uuid) -> Result<(), AppError> {
        let index = self.client.index(ProductSearchDocument::INDEX_NAME);

        index.delete_document(id).await.map_err(|e| {
            println!("Meilisearch Delete Error: {:?}", e);
            AppError::InternalServerError("Failed to delete product from index".into())
        })?;

        Ok(())
    }

    pub async fn search_products(
        &self,
        params: SearchQuery,
    ) -> Result<Vec<ProductSearchDocument>, AppError> {
        let index = self.client.index(ProductSearchDocument::INDEX_NAME);
        let sort_criteria = params.sort.as_deref().map(|s| [s]);
        let mut search_builder = index.search();
        
        if let Some(query_str) = &params.q {
            search_builder.with_query(query_str);
        }
        if let Some(filter_str) = &params.filter {
            search_builder.with_filter(filter_str);
        }
        if let Some(sort) = &sort_criteria {
            search_builder.with_sort(sort);
        }
        if let Some(limit) = params.limit {
            search_builder.with_limit(limit);
        }
        if let Some(offset) = params.offset {
            search_builder.with_offset(offset);
        }
        let search_results = search_builder
            .execute::<ProductSearchDocument>()
            .await
            .map_err(|e| {
                println!("Search error: {:?}", e);
                AppError::InternalServerError("Search failed".into())
            })?;

        // ดึงเฉพาะผลลัพธ์ออกมา
        let products: Vec<ProductSearchDocument> = search_results
            .hits
            .into_iter()
            .map(|result| result.result)
            .collect();

        Ok(products)
    }

    pub async fn setup_settings(&self) -> Result<(), AppError> {
        let index = self.client.index(ProductSearchDocument::INDEX_NAME);

        let settings = Settings::new()
            .with_searchable_attributes(&["name", "description"])
            .with_filterable_attributes(&["category_id", "price", "id"])
            .with_sortable_attributes(&["price"]);

        index.set_settings(&settings).await.map_err(|e| {
            println!("Failed to update settings: {:?}", e);
            AppError::InternalServerError("Failed to setup Meilisearch settings".into())
        })?;

        println!("Meilisearch settings updated!");
        Ok(())
    }

    pub async fn delete_all_documents(&self) -> Result<(), AppError> {
        let index = self.client.index(ProductSearchDocument::INDEX_NAME);

        let task = index
            .delete_all_documents()
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        // ตรงนี้สำคัญ! ถ้าจะล้างบาง ควร "รอ" ให้มันลบเสร็จจริง ๆ ก่อนค่อยเติมของใหม่
        // ไม่งั้นเดี๋ยวของใหม่เข้าไปปนกับของเก่าที่กำลังทยอยลบ
        task.wait_for_completion(&self.client, None, None)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(())
    }
}
