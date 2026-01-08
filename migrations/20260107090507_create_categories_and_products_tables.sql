-- ตาราง Categories
CREATE TABLE categories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL UNIQUE, -- ชื่อหมวดหมู่ห้ามซ้ำ

    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ
);

-- ตาราง Products
CREATE TABLE products (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Foreign Key เชื่อมไปหา Categories
    category_id UUID NOT NULL REFERENCES categories(id) ON DELETE RESTRICT,
    
    name TEXT NOT NULL,
    description TEXT, -- เป็น Nullable
    price DECIMAL(10, 2) NOT NULL, -- แนะนำใช้ DECIMAL แทน Float สำหรับเรื่องเงิน
    stock INT NOT NULL DEFAULT 0,
    
    -- ฟิลด์สถิติ (ตัด Relation ตารางอื่นออก แต่เก็บตัวเลขรวมไว้ตาม Schema)
    average_rating DOUBLE PRECISION NOT NULL DEFAULT 0,
    review_count INT NOT NULL DEFAULT 0,
    
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ
);

CREATE INDEX idx_products_category_id ON products(category_id);