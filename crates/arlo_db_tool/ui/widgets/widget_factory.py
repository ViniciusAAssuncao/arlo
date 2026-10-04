import tkinter as tk
from tkinter import ttk
from typing import Any, Callable, Dict, List, Optional, Tuple
import uuid
from arlo_db_tool.app_state import AppState
from arlo_db_tool.db.reference_lookup import (
    get_calendar_months,
    get_calendar_options,
    get_options,
)
from arlo_db_tool.schema.field_spec import FieldSpec, FieldType
from arlo_db_tool.utils.date_helpers import (
    DEFAULT_MONTHS,
    date_to_unix_seconds,
    unix_seconds_to_date,
)

class FieldWidget:
    def __init__(
        self,
        container: tk.Widget,
        get_value_fn: Callable[[], Any],
        set_value_fn: Callable[[Any], None],
        reload_fn: Optional[Callable[[], None]] = None,
        set_enabled_fn: Optional[Callable[[bool], None]] = None,
    ):
        self.container = container
        self.get_value = get_value_fn
        self.set_value = set_value_fn
        self.reload = reload_fn or (lambda: None)
        self.set_enabled = set_enabled_fn or (lambda enabled: None)

def create_field_widget(parent: tk.Widget, field: FieldSpec, app_state: AppState) -> FieldWidget:
    container = ttk.Frame(parent)

    if field.field_type == FieldType.UUID_PK:
        entry_var = tk.StringVar(value=str(uuid.uuid4()))
        entry = ttk.Entry(container, textvariable=entry_var, width=38)
        entry.pack(side=tk.LEFT, fill=tk.X, expand=True, padx=(0, 4))

        def generate_uuid():
            entry_var.set(str(uuid.uuid4()))

        btn = ttk.Button(container, text="Novo UUID", command=generate_uuid)
        btn.pack(side=tk.RIGHT)

        def get_val():
            val = entry_var.get().strip()
            return val if val else None

        def set_val(val):
            entry_var.set(str(val) if val is not None else "")

        def set_enabled(enabled):
            entry.config(state="normal" if enabled else "disabled")
            btn.config(state="normal" if enabled else "disabled")

        return FieldWidget(container, get_val, set_val, set_enabled_fn=set_enabled)

    elif field.field_type == FieldType.UUID_FK:
        cb_var = tk.StringVar()
        combo = ttk.Combobox(container, textvariable=cb_var, state="readonly", width=42)
        combo.pack(side=tk.LEFT, fill=tk.X, expand=True)

        options_map: Dict[str, Optional[str]] = {}
        reverse_map: Dict[str, str] = {}

        def reload_fk():
            nonlocal options_map, reverse_map
            options_map.clear()
            reverse_map.clear()

            target_table = field.fk_target_table or ""
            target_col = field.fk_target_column or "id"
            raw_options = get_options(
                app_state.database_path,
                target_table,
                id_col=target_col,
                where=field.fk_where,
            )

            display_list = []
            if field.nullable:
                display_list.append("(Vazio / NULL)")
                options_map["(Vazio / NULL)"] = None

            for row_id, label in raw_options:
                display_text = f"{label} [{row_id}]"
                display_list.append(display_text)
                options_map[display_text] = row_id
                reverse_map[str(row_id)] = display_text

            combo["values"] = display_list
            current_selection = cb_var.get()
            if current_selection not in display_list:
                if field.nullable:
                    cb_var.set("(Vazio / NULL)")
                elif display_list:
                    cb_var.set(display_list[0])
                else:
                    cb_var.set("")

        reload_fk()

        def get_val():
            sel = cb_var.get()
            return options_map.get(sel, None)

        def set_val(val):
            if val is None or str(val).strip() == "":
                if field.nullable:
                    cb_var.set("(Vazio / NULL)")
                else:
                    cb_var.set("")
            else:
                str_val = str(val)
                if str_val in reverse_map:
                    cb_var.set(reverse_map[str_val])
                else:
                    custom_label = f"[{str_val}]"
                    combo["values"] = list(combo["values"]) + [custom_label]
                    options_map[custom_label] = str_val
                    reverse_map[str_val] = custom_label
                    cb_var.set(custom_label)

        def set_enabled(enabled):
            combo.config(state="readonly" if enabled else "disabled")

        return FieldWidget(container, get_val, set_val, reload_fn=reload_fk, set_enabled_fn=set_enabled)

    elif field.field_type == FieldType.ENUM:
        cb_var = tk.StringVar()
        combo = ttk.Combobox(container, textvariable=cb_var, state="readonly", width=42)
        combo.pack(side=tk.LEFT, fill=tk.X, expand=True)

        enum_list = list(field.enum_values or [])
        if field.nullable:
            enum_list = ["(Vazio / NULL)"] + enum_list
        combo["values"] = enum_list

        if enum_list:
            default_val = str(field.default) if field.default is not None else enum_list[0]
            cb_var.set(default_val)

        def get_val():
            sel = cb_var.get()
            if sel == "(Vazio / NULL)" or not sel.strip():
                return None
            return sel

        def set_val(val):
            if val is None or str(val).strip() == "":
                if field.nullable:
                    cb_var.set("(Vazio / NULL)")
                else:
                    cb_var.set("")
            else:
                cb_var.set(str(val))

        def set_enabled(enabled):
            combo.config(state="readonly" if enabled else "disabled")

        return FieldWidget(container, get_val, set_val, set_enabled_fn=set_enabled)

    elif field.field_type == FieldType.BOOLEAN:
        bool_var = tk.IntVar(value=1 if field.default else 0)
        chk = ttk.Checkbutton(container, text="Ativo / Sim", variable=bool_var)
        chk.pack(side=tk.LEFT)

        def get_val():
            return 1 if bool_var.get() == 1 else 0

        def set_val(val):
            if val in (1, True, "1", "true", "True"):
                bool_var.set(1)
            else:
                bool_var.set(0)

        def set_enabled(enabled):
            chk.config(state="normal" if enabled else "disabled")

        return FieldWidget(container, get_val, set_val, set_enabled_fn=set_enabled)

    elif field.field_type == FieldType.UNIX_TIMESTAMP:
        top_row = ttk.Frame(container)
        top_row.pack(fill=tk.X, pady=(0, 4))

        lbl_cal = ttk.Label(top_row, text="Calendário:")
        lbl_cal.pack(side=tk.LEFT, padx=(0, 4))

        cal_var = tk.StringVar()
        combo_cal = ttk.Combobox(top_row, textvariable=cal_var, state="readonly", width=28)
        combo_cal.pack(side=tk.LEFT, padx=(0, 10))

        only_year_var = tk.BooleanVar(value=False)
        chk_only_year = ttk.Checkbutton(top_row, text="Apenas Ano", variable=only_year_var)
        chk_only_year.pack(side=tk.LEFT)

        inputs_row = ttk.Frame(container)
        inputs_row.pack(fill=tk.X)

        lbl_day = ttk.Label(inputs_row, text="Dia:")
        lbl_day.pack(side=tk.LEFT, padx=(0, 4))

        day_var = tk.StringVar(value="1")
        spin_day = ttk.Spinbox(inputs_row, from_=1, to=31, textvariable=day_var, width=4)
        spin_day.pack(side=tk.LEFT, padx=(0, 8))

        lbl_month = ttk.Label(inputs_row, text="Mês:")
        lbl_month.pack(side=tk.LEFT, padx=(0, 4))

        month_var = tk.StringVar(value=DEFAULT_MONTHS[0][1])
        combo_month = ttk.Combobox(inputs_row, textvariable=month_var, state="readonly", width=18)
        combo_month.pack(side=tk.LEFT, padx=(0, 8))

        lbl_year = ttk.Label(inputs_row, text="Ano:")
        lbl_year.pack(side=tk.LEFT, padx=(0, 4))

        year_var = tk.StringVar(value="2000")
        spin_year = ttk.Spinbox(inputs_row, from_=1, to=9999, textvariable=year_var, width=7)
        spin_year.pack(side=tk.LEFT)

        cal_options_map: Dict[str, str] = {}
        month_map: Dict[str, int] = {}
        reverse_month_map: Dict[int, str] = {}

        def _update_year_only_mode():
            if only_year_var.get():
                spin_day.config(state="disabled")
                combo_month.config(state="disabled")
            else:
                spin_day.config(state="normal")
                combo_month.config(state="readonly")

        chk_only_year.config(command=_update_year_only_mode)

        def _reload_months_for_calendar(calendar_id: Optional[str]):
            nonlocal month_map, reverse_month_map
            month_map.clear()
            reverse_month_map.clear()

            db_months = get_calendar_months(app_state.database_path, calendar_id)
            source_months = db_months if db_months else DEFAULT_MONTHS

            display_months = []
            for idx, label in source_months:
                disp = f"{idx:02d} - {label}" if not label.startswith(f"{idx:02d}") and not label.startswith(f"{idx} ") else label
                display_months.append(disp)
                month_map[disp] = idx
                reverse_month_map[idx] = disp

            combo_month["values"] = display_months
            cur_month_str = month_var.get()
            if cur_month_str not in display_months:
                if display_months:
                    month_var.set(display_months[0])
                else:
                    month_var.set("")

        def _on_cal_select(_event):
            sel_cal = cal_var.get()
            cal_id = cal_options_map.get(sel_cal)
            _reload_months_for_calendar(cal_id)

        combo_cal.bind("<<ComboboxSelected>>", _on_cal_select)

        def reload_timestamp_widget():
            cal_options_map.clear()
            cal_options = get_calendar_options(app_state.database_path)

            display_cals = []
            for cid, cname in cal_options:
                disp = f"{cname} [{cid}]"
                display_cals.append(disp)
                cal_options_map[disp] = cid

            combo_cal["values"] = display_cals
            if display_cals:
                if cal_var.get() not in display_cals:
                    cal_var.set(display_cals[0])
                selected_cid = cal_options_map.get(cal_var.get())
                _reload_months_for_calendar(selected_cid)
            else:
                cal_var.set("")
                _reload_months_for_calendar(None)

        reload_timestamp_widget()

        def get_val():
            try:
                y = int(year_var.get().strip())
            except (ValueError, TypeError):
                y = 2000

            if only_year_var.get():
                m = 1
                d = 1
            else:
                try:
                    d = int(day_var.get().strip())
                except (ValueError, TypeError):
                    d = 1

                sel_m_label = month_var.get()
                m = month_map.get(sel_m_label, 1)

            return date_to_unix_seconds(y, m, d)

        def set_val(val):
            if val is None or str(val).strip() == "":
                year_var.set("2000")
                day_var.set("1")
                if reverse_month_map:
                    month_var.set(reverse_month_map.get(1, list(reverse_month_map.values())[0]))
                only_year_var.set(False)
            else:
                try:
                    ts = int(val)
                except (ValueError, TypeError):
                    ts = 0
                y, m, d = unix_seconds_to_date(ts)
                year_var.set(str(y))
                day_var.set(str(d))

                if m in reverse_month_map:
                    month_var.set(reverse_month_map[m])
                elif reverse_month_map:
                    fallback_disp = f"{m:02d} - Mês {m}"
                    combo_month["values"] = list(combo_month["values"]) + [fallback_disp]
                    month_map[fallback_disp] = m
                    reverse_month_map[m] = fallback_disp
                    month_var.set(fallback_disp)

            _update_year_only_mode()

        def set_enabled(enabled):
            combo_cal.config(state="readonly" if enabled else "disabled")
            chk_only_year.config(state="normal" if enabled else "disabled")
            spin_year.config(state="normal" if enabled else "disabled")
            if enabled:
                _update_year_only_mode()
            else:
                spin_day.config(state="disabled")
                combo_month.config(state="disabled")

        return FieldWidget(container, get_val, set_val, reload_fn=reload_timestamp_widget, set_enabled_fn=set_enabled)

    elif field.field_type == FieldType.INTEGER:
        entry_var = tk.StringVar(value=str(field.default) if field.default is not None else "")
        entry = ttk.Entry(container, textvariable=entry_var, width=42)
        entry.pack(side=tk.LEFT, fill=tk.X, expand=True)

        def get_val():
            txt = entry_var.get().strip()
            if not txt:
                return None
            try:
                return int(txt)
            except ValueError:
                return txt

        def set_val(val):
            entry_var.set(str(val) if val is not None else "")

        def set_enabled(enabled):
            entry.config(state="normal" if enabled else "disabled")

        return FieldWidget(container, get_val, set_val, set_enabled_fn=set_enabled)

    elif field.field_type == FieldType.REAL:
        entry_var = tk.StringVar(value=str(field.default) if field.default is not None else "")
        entry = ttk.Entry(container, textvariable=entry_var, width=42)
        entry.pack(side=tk.LEFT, fill=tk.X, expand=True)

        def get_val():
            txt = entry_var.get().strip()
            if not txt:
                return None
            try:
                return float(txt)
            except ValueError:
                return txt

        def set_val(val):
            entry_var.set(str(val) if val is not None else "")

        def set_enabled(enabled):
            entry.config(state="normal" if enabled else "disabled")

        return FieldWidget(container, get_val, set_val, set_enabled_fn=set_enabled)

    else:
        entry_var = tk.StringVar(value=str(field.default) if field.default is not None else "")
        entry = ttk.Entry(container, textvariable=entry_var, width=42)
        entry.pack(side=tk.LEFT, fill=tk.X, expand=True)

        def get_val():
            txt = entry_var.get().strip()
            return txt if txt else None

        def set_val(val):
            entry_var.set(str(val) if val is not None else "")

        def set_enabled(enabled):
            entry.config(state="normal" if enabled else "disabled")

        return FieldWidget(container, get_val, set_val, set_enabled_fn=set_enabled)