import tkinter as tk
from tkinter import ttk
from typing import List, Tuple


class BulkGenerationPreviewPanel(ttk.LabelFrame):
    def __init__(self, parent: tk.Widget, title: str = "Prévia do Lote a Gerar"):
        super().__init__(parent, text=title, padding=(10, 8))

        self.lbl_summary = ttk.Label(
            self,
            text="Nenhum lote gerado. Configure os parâmetros e clique em 'Gerar Lote / SQL'.",
            font=("TkDefaultFont", 9, "bold"),
        )
        self.lbl_summary.pack(anchor="w", pady=(0, 4))

        self.metrics_container = ttk.Frame(self)
        self.metrics_container.pack(fill=tk.X, expand=True)

    def update_preview(self, summary_text: str, metrics: List[Tuple[str, str]]):
        self.lbl_summary.config(text=summary_text)
        for child in self.metrics_container.winfo_children():
            child.destroy()

        for idx, (label, val) in enumerate(metrics):
            row = idx // 2
            col = (idx % 2) * 2
            lbl_k = ttk.Label(self.metrics_container, text=f"{label}:", font=("TkDefaultFont", 9, "bold"))
            lbl_k.grid(row=row, column=col, sticky="w", padx=(4, 6), pady=2)
            lbl_v = ttk.Label(self.metrics_container, text=val)
            lbl_v.grid(row=row, column=col + 1, sticky="w", padx=(0, 16), pady=2)

    def clear(self):
        self.lbl_summary.config(text="Nenhum lote gerado.")
        for child in self.metrics_container.winfo_children():
            child.destroy()