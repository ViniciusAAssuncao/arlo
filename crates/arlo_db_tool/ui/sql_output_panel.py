import tkinter as tk
from tkinter import filedialog, messagebox, scrolledtext, ttk

class SqlOutputPanel(ttk.Frame):
    def __init__(self, parent: tk.Widget):
        super().__init__(parent, padding=(8, 8))

        lbl = ttk.Label(self, text="SQL Gerado:")
        lbl.pack(anchor="w", padx=(0, 0), pady=(0, 4))

        self.text_area = scrolledtext.ScrolledText(self, height=7, wrap=tk.WORD)
        self.text_area.pack(fill=tk.BOTH, expand=True, pady=(0, 6))

        btn_bar = ttk.Frame(self)
        btn_bar.pack(fill=tk.X)

        self.btn_copy = ttk.Button(btn_bar, text="Copiar", command=self._copy_to_clipboard)
        self.btn_copy.pack(side=tk.LEFT, padx=(0, 8))

        self.btn_save = ttk.Button(btn_bar, text="Salvar em arquivo .sql", command=self._save_to_file)
        self.btn_save.pack(side=tk.LEFT)

    def set_sql(self, sql_text: str):
        self.text_area.delete("1.0", tk.END)
        self.text_area.insert(tk.END, sql_text)

    def get_sql(self) -> str:
        return self.text_area.get("1.0", tk.END).strip()

    def _copy_to_clipboard(self):
        sql = self.get_sql()
        if not sql:
            messagebox.showinfo("Aviso", "Nenhum SQL gerado para copiar.")
            return
        self.clipboard_clear()
        self.clipboard_append(sql)
        messagebox.showinfo("Sucesso", "SQL copiado para a área de transferência!")

    def _save_to_file(self):
        sql = self.get_sql()
        if not sql:
            messagebox.showinfo("Aviso", "Nenhum SQL gerado para salvar.")
            return

        file_path = filedialog.asksaveasfilename(
            title="Salvar SQL",
            defaultextension=".sql",
            filetypes=[("SQL Files", "*.sql"), ("All Files", "*.*")],
        )
        if file_path:
            with open(file_path, "a", encoding="utf-8") as f:
                f.write(sql + "\n\n")
            messagebox.showinfo("Sucesso", f"SQL salvo com sucesso em:\n{file_path}")