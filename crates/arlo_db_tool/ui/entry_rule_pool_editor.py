import tkinter as tk
from tkinter import ttk
from typing import Callable, List, Tuple
import uuid
from arlo_db_tool.app_state import AppState
from arlo_db_tool.db.reference_lookup import get_options
from arlo_db_tool.domain.entry_rule_pool_draft import EntryRulePoolDraft

POOL_KINDS = [
    "AllTeams",
    "TopN",
    "BottomN",
    "GroupWinners",
    "GroupRunnersUp",
    "BestAtGroupPosition",
    "PositionRange",
    "ExternalCompetitionWinner",
]

class SinglePoolRow(ttk.Frame):
    def __init__(
        self,
        parent: tk.Widget,
        pool: EntryRulePoolDraft,
        external_competitions: List[Tuple[str, str]],
        on_delete: Callable[[], None],
        on_change: Callable[[], None],
    ):
        super().__init__(parent, padding=(4, 4))
        self.pool = pool
        self.external_competitions = external_competitions
        self.on_delete = on_delete
        self.on_change = on_change

        self.kind_var = tk.StringVar(value=pool.pool_kind)
        self.combo_kind = ttk.Combobox(
            self,
            textvariable=self.kind_var,
            values=POOL_KINDS,
            state="readonly",
            width=22,
        )
        self.combo_kind.pack(side=tk.LEFT, padx=(0, 6))
        self.combo_kind.bind("<<ComboboxSelected>>", self._on_kind_change)

        self.dyn_container = ttk.Frame(self)
        self.dyn_container.pack(side=tk.LEFT, fill=tk.X, expand=True)

        self.btn_del = ttk.Button(self, text="✕", width=3, command=self.on_delete)
        self.btn_del.pack(side=tk.RIGHT, padx=(6, 0))

        self._build_dynamic_fields()

    def _on_kind_change(self, _event):
        self.pool.pool_kind = self.kind_var.get()
        self._build_dynamic_fields()
        self.on_change()

    def update_competitions(self, comps: List[Tuple[str, str]]):
        self.external_competitions = comps
        self._build_dynamic_fields()

    def _build_dynamic_fields(self):
        for child in self.dyn_container.winfo_children():
            child.destroy()

        kind = self.pool.pool_kind

        if kind in ("TopN", "BottomN"):
            ttk.Label(self.dyn_container, text="Qtd:").pack(side=tk.LEFT, padx=(0, 4))
            count_var = tk.StringVar(value=str(self.pool.count if self.pool.count is not None else 4))

            def _sync_count(*_):
                try:
                    self.pool.count = int(count_var.get())
                except ValueError:
                    self.pool.count = None
                self.on_change()

            count_var.trace_add("write", _sync_count)
            spin = ttk.Spinbox(self.dyn_container, from_=1, to=200, textvariable=count_var, width=6)
            spin.pack(side=tk.LEFT)

        elif kind == "BestAtGroupPosition":
            ttk.Label(self.dyn_container, text="Posição (0=1º):").pack(side=tk.LEFT, padx=(0, 4))
            pos_var = tk.StringVar(value=str(self.pool.position_index if self.pool.position_index is not None else 0))

            def _sync_pos(*_):
                try:
                    self.pool.position_index = int(pos_var.get())
                except ValueError:
                    self.pool.position_index = None
                self.on_change()

            pos_var.trace_add("write", _sync_pos)
            spin_pos = ttk.Spinbox(self.dyn_container, from_=0, to=50, textvariable=pos_var, width=5)
            spin_pos.pack(side=tk.LEFT, padx=(0, 8))

            ttk.Label(self.dyn_container, text="Qtd:").pack(side=tk.LEFT, padx=(0, 4))
            count_var = tk.StringVar(value=str(self.pool.count if self.pool.count is not None else 2))

            def _sync_count(*_):
                try:
                    self.pool.count = int(count_var.get())
                except ValueError:
                    self.pool.count = None
                self.on_change()

            count_var.trace_add("write", _sync_count)
            spin_cnt = ttk.Spinbox(self.dyn_container, from_=1, to=200, textvariable=count_var, width=6)
            spin_cnt.pack(side=tk.LEFT)

        elif kind == "PositionRange":
            ttk.Label(self.dyn_container, text="Início:").pack(side=tk.LEFT, padx=(0, 4))
            start_var = tk.StringVar(value=str(self.pool.range_start_position if self.pool.range_start_position is not None else 1))

            def _sync_start(*_):
                try:
                    self.pool.range_start_position = int(start_var.get())
                except ValueError:
                    self.pool.range_start_position = None
                self.on_change()

            start_var.trace_add("write", _sync_start)
            spin_start = ttk.Spinbox(self.dyn_container, from_=1, to=200, textvariable=start_var, width=5)
            spin_start.pack(side=tk.LEFT, padx=(0, 8))

            ttk.Label(self.dyn_container, text="Fim:").pack(side=tk.LEFT, padx=(0, 4))
            end_var = tk.StringVar(value=str(self.pool.range_end_position if self.pool.range_end_position is not None else 8))

            def _sync_end(*_):
                try:
                    self.pool.range_end_position = int(end_var.get())
                except ValueError:
                    self.pool.range_end_position = None
                self.on_change()

            end_var.trace_add("write", _sync_end)
            spin_end = ttk.Spinbox(self.dyn_container, from_=1, to=200, textvariable=end_var, width=5)
            spin_end.pack(side=tk.LEFT)

        elif kind == "ExternalCompetitionWinner":
            ttk.Label(self.dyn_container, text="Competição:").pack(side=tk.LEFT, padx=(0, 4))
            comp_map = {f"{lbl} [{cid}]": cid for cid, lbl in self.external_competitions}
            reverse_map = {cid: f"{lbl} [{cid}]" for cid, lbl in self.external_competitions}

            current_sel = reverse_map.get(self.pool.external_competition_id or "", "")
            comp_var = tk.StringVar(value=current_sel)

            def _sync_comp(_event):
                sel = comp_var.get()
                self.pool.external_competition_id = comp_map.get(sel, None)
                self.on_change()

            cb = ttk.Combobox(
                self.dyn_container,
                textvariable=comp_var,
                values=list(comp_map.keys()),
                state="readonly",
                width=26,
            )
            cb.pack(side=tk.LEFT)
            cb.bind("<<ComboboxSelected>>", _sync_comp)

class EntryRulePoolEditor(ttk.LabelFrame):
    def __init__(
        self,
        parent: tk.Widget,
        app_state: AppState,
        pools: List[EntryRulePoolDraft],
        on_change: Callable[[], None],
    ):
        super().__init__(parent, text="Pools de Entrada / Qualificação (Entry Rules)", padding=(8, 6))
        self.app_state = app_state
        self.pools = pools
        self.on_change = on_change
        self.external_competitions: List[Tuple[str, str]] = []
        self.row_widgets: List[SinglePoolRow] = []

        top_bar = ttk.Frame(self)
        top_bar.pack(fill=tk.X, pady=(0, 4))

        self.btn_add = ttk.Button(top_bar, text="+ Adicionar Pool", command=self._add_pool)
        self.btn_add.pack(side=tk.LEFT)

        self.rows_container = ttk.Frame(self)
        self.rows_container.pack(fill=tk.BOTH, expand=True)

        self.reload_references()

    def reload_references(self):
        self.external_competitions = get_options(
            self.app_state.database_path,
            "competitions",
            id_col="id",
            label_expr="name",
        )
        for row in self.row_widgets:
            row.update_competitions(self.external_competitions)

    def _render_rows(self):
        for child in self.rows_container.winfo_children():
            child.destroy()
        self.row_widgets.clear()

        for idx, pool in enumerate(self.pools):
            pool.pool_order_index = idx
            row_w = SinglePoolRow(
                self.rows_container,
                pool,
                self.external_competitions,
                on_delete=lambda p=pool: self._delete_pool(p),
                on_change=self.on_change,
            )
            row_w.pack(fill=tk.X, expand=True, pady=2)
            self.row_widgets.append(row_w)

    def _add_pool(self):
        new_pool = EntryRulePoolDraft(
            id=str(uuid.uuid4()),
            pool_order_index=len(self.pools),
            pool_kind="AllTeams",
        )
        self.pools.append(new_pool)
        self._render_rows()
        self.on_change()

    def _delete_pool(self, target_pool: EntryRulePoolDraft):
        self.pools.remove(target_pool)
        self._render_rows()
        self.on_change()

    def set_pools(self, pools: List[EntryRulePoolDraft]):
        self.pools = pools
        self._render_rows()