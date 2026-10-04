import copy
import tkinter as tk
from tkinter import messagebox, ttk
from typing import List, Optional, Tuple
from arlo_db_tool.app_state import AppState
from arlo_db_tool.domain.draft_factory import create_default_league_calendar_config_draft
from arlo_db_tool.domain.league_calendar_config_draft import LeagueCalendarConfigDraft
from arlo_db_tool.loader.aggregate_loader import load_league_calendar_config_draft_from_db
from arlo_db_tool.sql.aggregate_delete_builder import generate_full_delete_statements
from arlo_db_tool.sql.aggregate_statement_builder import gerar_statements_create
from arlo_db_tool.sql.aggregate_update_builder import generate_update_statements
from arlo_db_tool.ui.config_core_panel import ConfigCorePanel
from arlo_db_tool.ui.group_list_panel import GroupListPanel
from arlo_db_tool.ui.league_calendar_config_operation_bar import LeagueCalendarConfigOperationBar
from arlo_db_tool.ui.sql_output_panel import SqlOutputPanel
from arlo_db_tool.ui.stage_list_panel import StageListPanel
from arlo_db_tool.ui.tie_break_criteria_panel import TieBreakCriteriaPanel
from arlo_db_tool.ui.weekday_list_panel import WeekdayListPanel
from arlo_db_tool.validation.aggregate_validator import validar

class LeagueCalendarConfigScreen(ttk.Frame):
    def __init__(self, parent: tk.Widget, app_state: AppState):
        super().__init__(parent)
        self.app_state = app_state
        self.draft: LeagueCalendarConfigDraft = create_default_league_calendar_config_draft()
        self.original_draft: Optional[LeagueCalendarConfigDraft] = None

        top_actions = ttk.Frame(self, padding=(10, 8))
        top_actions.pack(fill=tk.X)

        title_lbl = ttk.Label(
            top_actions,
            text="Configurador de Calendário de Liga (League Calendar Config)",
            font=("TkDefaultFont", 12, "bold"),
        )
        title_lbl.pack(side=tk.LEFT)

        self.btn_reset = ttk.Button(top_actions, text="Resetar Rascunho", command=self._reset_draft)
        self.btn_reset.pack(side=tk.RIGHT, padx=4)

        self.btn_reload = ttk.Button(top_actions, text="Recarregar Referências", command=self.reload_references)
        self.btn_reload.pack(side=tk.RIGHT, padx=4)

        self.btn_validate_gen = ttk.Button(
            top_actions,
            text="Validar & Gerar SQL",
            command=self._validate_and_generate_sql,
        )
        self.btn_validate_gen.pack(side=tk.RIGHT, padx=4)

        sep0 = ttk.Separator(self, orient=tk.HORIZONTAL)
        sep0.pack(fill=tk.X)

        self.op_bar = LeagueCalendarConfigOperationBar(
            self,
            self.app_state,
            on_load_config=self._on_load_config,
        )
        self.op_bar.pack(fill=tk.X)

        sep = ttk.Separator(self, orient=tk.HORIZONTAL)
        sep.pack(fill=tk.X)

        self.canvas = tk.Canvas(self, borderwidth=0, highlightthickness=0)
        self.scrollbar = ttk.Scrollbar(self, orient=tk.VERTICAL, command=self.canvas.yview)
        self.scroll_content = ttk.Frame(self.canvas, padding=(8, 8))

        self.scroll_content.bind(
            "<Configure>",
            lambda _e: self.canvas.configure(scrollregion=self.canvas.bbox("all")),
        )
        self.canvas_frame = self.canvas.create_window((0, 0), window=self.scroll_content, anchor="nw")
        self.canvas.configure(xscrollcommand=None, yscrollcommand=self.scrollbar.set)

        self.canvas.bind(
            "<Configure>",
            lambda e: self.canvas.itemconfig(self.canvas_frame, width=e.width),
        )

        self.canvas.pack(side=tk.TOP, fill=tk.BOTH, expand=True)
        self.scrollbar.pack(side=tk.RIGHT, fill=tk.Y, before=self.canvas)

        self.core_panel = ConfigCorePanel(self.scroll_content, self.app_state)
        self.core_panel.pack(fill=tk.X, expand=True, pady=4)

        self.weekday_panel = WeekdayListPanel(self.scroll_content, self.app_state)
        self.weekday_panel.pack(fill=tk.X, expand=True, pady=4)

        self.tie_break_panel = TieBreakCriteriaPanel(self.scroll_content)
        self.tie_break_panel.pack(fill=tk.X, expand=True, pady=4)

        self.group_panel = GroupListPanel(
            self.scroll_content,
            self.app_state,
            on_groups_changed=self._on_groups_changed,
        )
        self.group_panel.pack(fill=tk.X, expand=True, pady=4)

        self.stage_panel = StageListPanel(
            self.scroll_content,
            self.app_state,
            get_draft_groups_fn=self._get_draft_groups,
            on_change=lambda: None,
        )
        self.stage_panel.pack(fill=tk.BOTH, expand=True, pady=4)

        sep_bottom = ttk.Separator(self, orient=tk.HORIZONTAL)
        sep_bottom.pack(fill=tk.X)

        self.sql_output = SqlOutputPanel(self)
        self.sql_output.pack(side=tk.BOTTOM, fill=tk.X)

        self._populate_all()

    def _on_load_config(self, config_id: str):
        if not config_id:
            messagebox.showerror("Erro", "Nenhuma configuração selecionada.")
            return

        loaded = load_league_calendar_config_draft_from_db(self.app_state.database_path, config_id)
        if loaded is None:
            messagebox.showerror("Erro", f"Não foi possível carregar a configuração ID: {config_id}")
            return

        self.draft = loaded
        self.original_draft = copy.deepcopy(loaded)
        self._populate_all()
        messagebox.showinfo("Sucesso", "Configuração carregada com sucesso.")

    def _get_draft_groups(self) -> List[Tuple[str, str]]:
        self.group_panel.sync_to_draft(self.draft)
        return [(g.id, g.name or f"Grupo #{g.order_index + 1}") for g in self.draft.groups]

    def _on_groups_changed(self):
        self.group_panel.sync_to_draft(self.draft)
        self.stage_panel.reload_groups()

    def _populate_all(self):
        self.core_panel.populate_from_draft(self.draft)
        self.weekday_panel.reload_references()
        self.weekday_panel.populate_from_draft(self.draft)
        self.tie_break_panel.populate_from_draft(self.draft)
        self.group_panel.populate_from_draft(self.draft)
        self.stage_panel.populate_from_draft(self.draft)

    def _sync_all(self):
        self.core_panel.sync_to_draft(self.draft)
        self.weekday_panel.sync_to_draft(self.draft)
        self.tie_break_panel.sync_to_draft(self.draft)
        self.group_panel.sync_to_draft(self.draft)
        self.stage_panel.sync_to_draft(self.draft)
        self.draft.renumber()

    def _reset_draft(self):
        if messagebox.askyesno("Confirmar Reset", "Deseja realmente resetar o rascunho da configuração para os valores padrão?"):
            self.draft = create_default_league_calendar_config_draft()
            self.original_draft = None
            self.op_bar.set_operation("Create")
            self._populate_all()
            self.sql_output.set_sql("")

    def reload_references(self):
        self.op_bar.reload_references()
        self.core_panel.reload_references()
        self.weekday_panel.reload_references()
        self.group_panel.reload_references()
        self.stage_panel.reload_references()

    def _validate_and_generate_sql(self):
        op = self.op_bar.get_operation()

        if op == "Create":
            self._sync_all()
            errors = validar(self.draft)
            if errors:
                messagebox.showerror("Erros de Validação da Configuração", "\n".join(errors))
                return

            statements = gerar_statements_create(self.draft)
            full_sql = "\n".join(statements)
            self.sql_output.set_sql(full_sql)
            messagebox.showinfo("Sucesso", f"{len(statements)} comandos SQL gerados com sucesso!")

        elif op == "Update":
            if self.original_draft is None:
                messagebox.showerror("Erro", "Para atualizar, primeiro carregue uma configuração existente.")
                return

            self._sync_all()
            errors = validar(self.draft)
            if errors:
                messagebox.showerror("Erros de Validação da Configuração", "\n".join(errors))
                return

            statements = generate_update_statements(self.original_draft, self.draft)
            if not statements:
                messagebox.showinfo("Aviso", "Nenhuma alteração detectada na configuração.")
                return

            full_sql = "\n".join(statements)
            self.sql_output.set_sql(full_sql)
            self.original_draft = copy.deepcopy(self.draft)
            messagebox.showinfo("Sucesso", f"{len(statements)} comandos SQL de atualização gerados com sucesso!")

        elif op == "Delete":
            if self.original_draft is None:
                messagebox.showerror("Erro", "Para deletar, primeiro carregue uma configuração existente.")
                return

            statements = generate_full_delete_statements(self.original_draft)
            full_sql = "\n".join(statements)
            self.sql_output.set_sql(full_sql)
            messagebox.showinfo("Sucesso", f"{len(statements)} comandos SQL de exclusão gerados com sucesso!")