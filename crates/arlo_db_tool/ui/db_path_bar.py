import tkinter as tk
from tkinter import filedialog, ttk
from typing import Callable
from arlo_db_tool.app_state import AppState

class DbPathBar(ttk.Frame):
    def __init__(
        self,
        parent: tk.Widget,
        app_state: AppState,
        on_db_changed: Callable[[str], None],
        on_reload_references: Callable[[], None],
    ):
        super().__init__(parent, padding=(8, 8))
        self.app_state = app_state
        self.on_db_changed = on_db_changed
        self.on_reload_references = on_reload_references

        self.path_var = tk.StringVar(value="(Nenhum banco selecionado)")

        label = ttk.Label(self, text="Banco SQLite:")
        label.pack(side=tk.LEFT, padx=(0, 6))

        self.entry = ttk.Entry(self, textvariable=self.path_var, state="readonly", width=50)
        self.entry.pack(side=tk.LEFT, fill=tk.X, expand=True, padx=(0, 6))

        self.btn_choose = ttk.Button(self, text="Escolher banco...", command=self._choose_db)
        self.btn_choose.pack(side=tk.LEFT, padx=(0, 6))

        self.btn_reload = ttk.Button(self, text="Recarregar referências", command=self._reload)
        self.btn_reload.pack(side=tk.LEFT)

    def _choose_db(self):
        chosen = filedialog.askopenfilename(
            title="Selecionar Banco SQLite",
            filetypes=[("SQLite Database", "*.db *.sqlite *.sqlite3"), ("All Files", "*.*")],
        )
        if chosen:
            self.app_state.database_path = chosen
            self.path_var.set(chosen)
            self.on_db_changed(chosen)

    def _reload(self):
        self.on_reload_references()