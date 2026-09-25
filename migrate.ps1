$tempMigrations = "target\all_migrations"
New-Item -ItemType Directory -Force -Path $tempMigrations | Out-Null

Copy-Item -Path ".\crates\arlo-controller\migrations\*" -Destination $tempMigrations -Force
Copy-Item -Path ".\migrations\*" -Destination $tempMigrations -Force

cargo sqlx migrate run --source $tempMigrations --database-url "sqlite://database/arlo.db"