import os
import sqlite3
from typing import Optional

def get_db_connection(db_path: Optional[str]) -> sqlite3.Connection:
    if not db_path:
        raise ValueError("Caminho do banco de dados não informado")
    if not os.path.exists(db_path):
        raise FileNotFoundError(f"Arquivo de banco de dados não encontrado: {db_path}")
    uri_path = f"file:{os.path.abspath(db_path)}?mode=ro"
    conn = sqlite3.connect(uri_path, uri=True)
    conn.row_factory = sqlite3.Row
    return conn