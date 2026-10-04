import tkinter as tk
from tkinter import ttk
from typing import Callable, Optional
from arlo_db_tool.app_state import AppState
from arlo_db_tool.schema.registry import get_all_entity_names

class EntitySelector(ttk.Frame):
    def __init__(
        self,
        parent: tk.Widget,
        app_state: AppState,
        on_entity_changed: Callable[[str], None],
    ):
        super().__init__(parent, padding=(8, 4))
        self.app_state = app_state
        self.on_entity_changed = on_entity_changed

        label = ttk.Label(self, text="Entidade:")
        label.pack(side=tk.LEFT, padx=(0, 6))

        entities = get_all_entity_names()
        self.selected_var = tk.StringVar()
        self.combo = ttk.Combobox(
            self,
            textvariable=self.selected_var,
            values=entities,
            state="readonly",
            width=26,
        )
        self.combo.pack(side=tk.LEFT)
        self.combo.bind("<<ComboboxSelected>>", self._on_select)

        if entities:
            default_entity = entities[0]
            self.selected_var.set(default_entity)
            self.app_state.current_entity = default_entity

    def _on_select(self, _event):
        chosen = self.selected_var.get()
        self.app_state.current_entity = chosen
        self.on_entity_changed(chosen)

    def get_selected_entity(self) -> Optional[str]:
        val = self.selected_var.get()
        return val if val else None