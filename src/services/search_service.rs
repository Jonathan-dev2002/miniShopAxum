use crate::models::{dto::ProductSearchDocument, error::AppError};
use meilisearch_sdk::{client::Client, search::SearchResults, settings::{self, Settings}};

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

    pub async fn search_products(
        &self,
        query: String,
    ) -> Result<Vec<ProductSearchDocument>, AppError> {
        let index = self.client.index(ProductSearchDocument::INDEX_NAME);

        let search_results = index
            .search()
            .with_query(&query)
            .execute::<ProductSearchDocument>()
            .await
            .map_err(|_| AppError::InternalServerError("Search failed".into()))?;

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

        index.set_settings(&settings).await
            .map_err(|e| {
            println!("Failed to update settings: {:?}", e);
            AppError::InternalServerError("Failed to setup Meilisearch settings".into())
        })?;

        println!("Meilisearch settings updated!");
        Ok(())
    }
}
