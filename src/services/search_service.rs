use meilisearch_sdk::client::Client;
use crate::models::{dto::ProductSearchDocument, error::AppError};

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
        
        // ส่งข้อมูลเป็น Batch (เร็วกว่าส่งทีละตัว)
        index.add_documents(products, Some("id"))
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
        index.add_documents(&[product], Some("id")).await
            .map_err(|_| AppError::InternalServerError("Failed to index product".into()))?;
            
        Ok(())
    }

    // ฟังก์ชันค้นหา
    pub async fn search_products(&self, query: String) -> Result<Vec<ProductSearchDocument>, AppError> {
        let index = self.client.index(ProductSearchDocument::INDEX_NAME);
        
        let search_results = index.search()
            .with_query(&query)
            .execute::<ProductSearchDocument>()
            .await
            .map_err(|_| AppError::InternalServerError("Search failed".into()))?;

        // ดึงเฉพาะผลลัพธ์ออกมา
        let products: Vec<ProductSearchDocument> = search_results.hits
            .into_iter()
            .map(|result| result.result)
            .collect();

        Ok(products)
    }
}