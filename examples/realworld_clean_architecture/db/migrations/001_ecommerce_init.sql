-- ==============================================================================
-- Kora E-Commerce Production Database Migration
-- Version: 001
-- Dialect: MySQL 8.0 / InnoDB
-- ==============================================================================

SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;

-- 1. Users Table
CREATE TABLE IF NOT EXISTS `users` (
    `id` BIGINT AUTO_INCREMENT PRIMARY KEY,
    `email` VARCHAR(255) NOT NULL UNIQUE,
    `full_name` VARCHAR(255) NOT NULL,
    `password_hash` VARCHAR(255) NOT NULL,
    `role` ENUM('customer', 'admin', 'moderator') NOT NULL DEFAULT 'customer',
    `balance` BIGINT NOT NULL DEFAULT 0,
    `is_active` BOOLEAN NOT NULL DEFAULT TRUE,
    `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    `updated_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX `idx_users_email` (`email`),
    INDEX `idx_users_role` (`role`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- 2. Products Table
CREATE TABLE IF NOT EXISTS `products` (
    `id` BIGINT AUTO_INCREMENT PRIMARY KEY,
    `sku` VARCHAR(64) NOT NULL UNIQUE,
    `title` VARCHAR(255) NOT NULL,
    `description` TEXT,
    `price_cents` BIGINT NOT NULL,
    `stock_quantity` INT NOT NULL DEFAULT 0,
    `is_published` BOOLEAN NOT NULL DEFAULT TRUE,
    `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    `updated_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX `idx_products_sku` (`sku`),
    INDEX `idx_products_price` (`price_cents`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- 3. Orders Table
CREATE TABLE IF NOT EXISTS `orders` (
    `id` BIGINT AUTO_INCREMENT PRIMARY KEY,
    `order_number` VARCHAR(64) NOT NULL UNIQUE,
    `user_id` BIGINT NOT NULL,
    `total_amount` BIGINT NOT NULL,
    `status` ENUM('pending', 'paid', 'shipped', 'delivered', 'cancelled') NOT NULL DEFAULT 'pending',
    `payment_status` ENUM('unpaid', 'authorized', 'captured', 'refunded') NOT NULL DEFAULT 'unpaid',
    `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    `updated_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    CONSTRAINT `fk_orders_user` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE CASCADE,
    INDEX `idx_orders_user` (`user_id`),
    INDEX `idx_orders_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- 4. Order Items Table
CREATE TABLE IF NOT EXISTS `order_items` (
    `id` BIGINT AUTO_INCREMENT PRIMARY KEY,
    `order_id` BIGINT NOT NULL,
    `product_id` BIGINT NOT NULL,
    `quantity` INT NOT NULL,
    `unit_price` BIGINT NOT NULL,
    `subtotal` BIGINT NOT NULL,
    CONSTRAINT `fk_order_items_order` FOREIGN KEY (`order_id`) REFERENCES `orders` (`id`) ON DELETE CASCADE,
    CONSTRAINT `fk_order_items_product` FOREIGN KEY (`product_id`) REFERENCES `products` (`id`) ON DELETE RESTRICT,
    INDEX `idx_order_items_order` (`order_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- 5. Rate Limits Tracking Table (for persistent fallback)
CREATE TABLE IF NOT EXISTS `rate_limit_logs` (
    `id` BIGINT AUTO_INCREMENT PRIMARY KEY,
    `ip_address` VARCHAR(64) NOT NULL,
    `endpoint` VARCHAR(255) NOT NULL,
    `request_count` INT NOT NULL DEFAULT 1,
    `window_start` TIMESTAMP NOT NULL,
    INDEX `idx_rate_limit_ip_window` (`ip_address`, `window_start`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- 6. Audit & Observability Logs Table
CREATE TABLE IF NOT EXISTS `audit_logs` (
    `id` BIGINT AUTO_INCREMENT PRIMARY KEY,
    `user_id` BIGINT NULL,
    `action` VARCHAR(128) NOT NULL,
    `resource` VARCHAR(128) NOT NULL,
    `ip_address` VARCHAR(64) NOT NULL,
    `status_code` INT NOT NULL,
    `duration_ms` INT NOT NULL,
    `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    INDEX `idx_audit_created` (`created_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Seed Initial Demo Data
INSERT INTO `users` (`email`, `full_name`, `password_hash`, `role`, `balance`, `is_active`)
VALUES
    ('ahmet@kora-lang.org', 'Ahmet Architect', '$argon2id$v=19$m=65536,t=3,p=4$dummyhash', 'admin', 500000, 1),
    ('customer@example.com', 'Sarah Connor', '$argon2id$v=19$m=65536,t=3,p=4$dummyhash', 'customer', 150000, 1)
ON DUPLICATE KEY UPDATE `email`=`email`;

INSERT INTO `products` (`sku`, `title`, `description`, `price_cents`, `stock_quantity`, `is_published`)
VALUES
    ('MACBOOK-M3-MAX', 'Apple MacBook Pro 16 M3 Max', 'Workstation laptop with 64GB unified RAM', 349900, 25, 1),
    ('SONY-WH1000XM5', 'Sony WH-1000XM5 Wireless Headphones', 'Noise-canceling premium over-ear headphones', 39900, 80, 1),
    ('KEYCHRON-Q1-PRO', 'Keychron Q1 Pro Wireless Mechanical Keyboard', 'Custom mechanical keyboard with hot-swappable switches', 19900, 120, 1)
ON DUPLICATE KEY UPDATE `sku`=`sku`;

SET FOREIGN_KEY_CHECKS = 1;

