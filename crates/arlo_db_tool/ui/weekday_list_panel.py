import tkinter as tk
from tkinter import ttk
from typing import Dict, List, Optional, Tuple
import uuid
from arlo_db_tool.app_state import AppState
from arlo_db_tool.db.reference_lookup import get_calendar_options, get_calendar_weekdays
from arlo_db_tool.domain.league_calendar_config_draft import LeagueCalendarConfigDraft
from arlo_db_tool.domain.weekday_draft import WeekdayDraft

class WeekdayListPanel(ttk.LabelFrame):
    def __init__(self, parent: tk.Widget, app_state: AppState):
        super().__init__(parent, text="Dias da Semana para Partidas", padding=(10, 8))
        self.app_state = app_state
        self.checkbox_vars: Dict[int, tk.BooleanVar] = {}
        self.calendar_map: Dict[str, str] = {}
        self.current_weekdays: List[Tuple[int, str]] = []

        top_bar = ttk.Frame(self)
        top_bar.pack(fill=tk.X, pady=(0, 8))

        lbl_cal = ttk.Label(top_bar, text="Calendário de Referência:")
        lbl_cal.pack(side=tk.LEFT, padx=(0, 6))

        self.cal_var = tk.StringVar()
        self.combo_cal = ttk.Combobox(
            top_bar,
            textvariable=self.cal_var,
            state="readonly",
            width=42,
        )
        self.combo_cal.pack(side=tk.LEFT, padx=(0, 12))
        self.combo_cal.bind("<<ComboboxSelected>>", self._on_calendar_changed)

        btn_select_all = ttk.Button(top_bar, text="Marcar Todos", command=self._select_all)
        btn_select_all.pack(side=tk.LEFT, padx=(0, 4))

        btn_deselect_all = ttk.Button(top_bar, text="Desmarcar Todos", command=self._deselect_all)
        btn_deselect_all.pack(side=tk.LEFT)

        self.grid_frame = ttk.Frame(self)
        self.grid_frame.pack(fill=tk.X)

        self.reload_references()

    def reload_references(self):
        cal_options = get_calendar_options(self.app_state.database_path)
        self.calendar_map.clear()

        display_values = []
        for cal_id, cal_name in cal_options:
            disp = f"{cal_name} [{cal_id}]"
            display_values.append(disp)
            self.calendar_map[disp] = cal_id

        self.combo_cal["values"] = display_values

        if display_values:
            if self.cal_var.get() not in display_values:
                self.cal_var.set(display_values[0])
            self._update_weekdays_from_database()
        else:
            self.cal_var.set("")
            self.current_weekdays = []
            self._render_checkboxes()

    def _on_calendar_changed(self, _event):
        self._update_weekdays_from_database()

    def _update_weekdays_from_database(self):
        selected_display = self.cal_var.get()
        cal_id = self.calendar_map.get(selected_display)

        if cal_id:
            db_weekdays = get_calendar_weekdays(self.app_state.database_path, cal_id)
            if db_weekdays:
                self.current_weekdays = [(idx, f"{idx} ({name})") for idx, name in db_weekdays]
            else:
                self.current_weekdays = []
        else:
            self.current_weekdays = []

        self._render_checkboxes()

    def _render_checkboxes(self):
        previous_checked = {idx for idx, var in self.checkbox_vars.items() if var.get()}

        for child in self.grid_frame.winfo_children():
            child.destroy()
        self.checkbox_vars.clear()

        if not self.current_weekdays:
            lbl_empty = ttk.Label(self.grid_frame, text="Nenhum dia da semana encontrado no banco de dados para este calendário.")
            lbl_empty.pack(anchor="w", padx=4, pady=4)
            return

        columns_count = 4
        for pos, (day_idx, label_text) in enumerate(self.current_weekdays):
            is_checked = day_idx in previous_checked
            var = tk.BooleanVar(value=is_checked)
            self.checkbox_vars[day_idx] = var
            chk = ttk.Checkbutton(self.grid_frame, text=label_text, variable=var)
            chk.grid(row=pos // columns_count, column=pos % columns_count, sticky="w", padx=8, pady=4)

    def _select_all(self):
        for var in self.checkbox_vars.values():
            var.set(True)

    def _deselect_all(self):
        for var in self.checkbox_vars.values():
            var.set(False)

    def populate_from_draft(self, draft: LeagueCalendarConfigDraft):
        if not self.current_weekdays:
            self._update_weekdays_from_database()

        selected_indices = {w.weekday_order_index for w in draft.weekdays}
        for day_idx, var in self.checkbox_vars.items():
            var.set(day_idx in selected_indices)

    def sync_to_draft(self, draft: LeagueCalendarConfigDraft):
        weekdays = []
        for day_idx in sorted(self.checkbox_vars.keys()):
            if self.checkbox_vars[day_idx].get():
                weekdays.append(WeekdayDraft(id=str(uuid.uuid4()), weekday_order_index=day_idx))
        draft.weekdays = weekdays