# Example 04: MySQL Database Connection & ORM Repository

## Overview
This example demonstrates connecting Kora backend services to a production relational database (MySQL):
- **Database Connection Pooling**: Built-in `DbPool` injection into repository services.
- **Prepared Statements**: Parameterized SQL queries (`?`) immune to SQL injection.
- **Transactional Updates**: Safely modifying balances and customer states.
- **Docker Compose Setup**: Ready-to-run containerized MySQL 8.0 instance with initialization script.
- **Clean Architecture Segregation**: Models in `models/`, interface contracts in `interfaces/`, repository implementation in `services/`.

## Directory Structure
```
04_mysql_database_orm/
├── README.md
├── main.kora                   # REST routes connected to MySQL repository
├── docker-compose.yml          # Containerized MySQL 8.0 database
├── mysql-init.sql              # Database schema initialization script
├── interfaces/
│   └── customer_repository.kora # Customer repository interface contract
├── models/
│   └── customer_models.kora     # Customer models and input DTOs
└── services/
    └── mysql_customer_service.kora # MySQL database queries & connection pool usage
```

## Running the Example
```bash
# 1. Start the MySQL database container
docker compose up -d

# 2. Type check the Kora application
kora check main.kora

# 3. Launch the Kora backend service
kora run main.kora

# 4. Lookup a customer by email
curl -X GET http://localhost:7582/customers/lookup -d "alice@kora.dev"

# 5. Insert a new customer
curl -X POST http://localhost:7582/customers -d "bob@kora.dev"

# 6. Add credit to customer account
curl -X POST http://localhost:7582/customers/1/credit
```

