# Example 13: Modular Monolith E-Commerce Platform

A production-ready modular monolith featuring isolated domain boundaries:

- **Modular Domain Architecture**:
  - `src/modules/catalog/`: Product discovery, pricing, inventory repository and routes.
  - `src/modules/billing/`: Customer charging, invoice creation, and payment auditing.
- **ORM Auto-Migration**:
  - [catalog_item.entity.kora](src/modules/catalog/models/catalog_item.entity.kora) -> `catalog_items` table.
  - [invoice.entity.kora](src/modules/billing/models/invoice.entity.kora) -> `invoices` table.
- **Database Seeder**: [ecommerce.seeder.kora](src/database/ecommerce.seeder.kora) for turnkey fixtures.
