use leptos::prelude::*;

// ─── Domain ──────────────────────────────────────────────────────────────────

/// Solve the 0/1 knapsack problem and return the full DP table.
/// table[i][w] = best value using items 0..i with capacity w.
fn knapsack_table(capacity: usize, weights: &[usize], benefits: &[usize]) -> Vec<Vec<usize>> {
    let n = weights.len();
    // (n+1) rows × (capacity+1) cols, row 0 is the "no items" baseline
    let mut table = vec![vec![0usize; capacity + 1]; n + 1];

    for i in 1..=n {
        let w = weights[i - 1];
        let b = benefits[i - 1];
        for c in 0..=capacity {
            table[i][c] = if w > c {
                table[i - 1][c]
            } else {
                table[i - 1][c].max(table[i - 1][c - w] + b)
            };
        }
    }
    table
}

// ─── Parsing helpers ─────────────────────────────────────────────────────────

fn parse_list(s: &str) -> Result<Vec<usize>, String> {
    s.split(',')
        .map(|t| {
            t.trim()
                .parse::<usize>()
                .map_err(|_| format!("'{}' is not a valid positive integer", t.trim()))
        })
        .collect()
}

// ─── Component ───────────────────────────────────────────────────────────────

#[component]
pub fn KnapsackVisualizer() -> impl IntoView {
    // ── form state ──────────────────────────────────────────────────────────
    let (capacity_input, set_capacity_input) = signal(String::from("6"));
    let (weights_input, set_weights_input) = signal(String::from("2, 3, 4"));
    let (benefits_input, set_benefits_input) = signal(String::from("3, 4, 5"));
    let (error_msg, set_error_msg) = signal(Option::<String>::None);

    // ── solver state ────────────────────────────────────────────────────────
    // The full DP table (rows = items+1, cols = capacity+1)
    let (dp_table, set_dp_table) = signal(Option::<Vec<Vec<usize>>>::None);
    // weights / benefits kept alongside the table for header rendering
    let (item_weights, set_item_weights) = signal(Vec::<usize>::new());
    let (item_benefits, set_item_benefits) = signal(Vec::<usize>::new());
    let (capacity, set_capacity) = signal(0usize);

    // How many *data* cells have been revealed (row-major, skipping row 0
    // which is the "0 items" baseline and is always shown).
    // A value of None means "all revealed" (Solve was pressed).
    let (revealed, set_revealed) = signal(Option::<usize>::Some(0));

    // ── helpers ─────────────────────────────────────────────────────────────

    // Total data cells = n_items × (capacity+1)
    let total_cells = move || {
        dp_table
            .get()
            .map(|t| (t.len().saturating_sub(1)) * t[0].len())
            .unwrap_or(0)
    };

    // ── Solve ────────────────────────────────────────────────────────────────
    let on_solve = move |_| {
        set_error_msg.set(None);

        let cap_str = capacity_input.get();
        let w_str = weights_input.get();
        let b_str = benefits_input.get();

        let cap = match cap_str.trim().parse::<usize>() {
            Ok(v) if v > 0 => v,
            _ => {
                set_error_msg.set(Some("Capacity (m) must be a positive integer.".into()));
                return;
            }
        };

        let ws = match parse_list(&w_str) {
            Ok(v) if !v.is_empty() => v,
            Err(e) => {
                set_error_msg.set(Some(format!("Weights: {e}")));
                return;
            }
            _ => {
                set_error_msg.set(Some("Enter at least one weight.".into()));
                return;
            }
        };

        let bs = match parse_list(&b_str) {
            Ok(v) => v,
            Err(e) => {
                set_error_msg.set(Some(format!("Benefits: {e}")));
                return;
            }
        };

        if ws.len() != bs.len() {
            set_error_msg.set(Some(format!(
                "Number of weights ({}) must equal number of benefits ({}).",
                ws.len(),
                bs.len()
            )));
            return;
        }

        let table = knapsack_table(cap, &ws, &bs);
        set_capacity.set(cap);
        set_item_weights.set(ws);
        set_item_benefits.set(bs);
        set_dp_table.set(Some(table));
        set_revealed.set(None); // reveal everything immediately
    };

    // ── Step-by-step ─────────────────────────────────────────────────────────
    let on_step = move |_| {
        set_error_msg.set(None);

        // If no table yet, parse inputs and initialise (reveal = 0)
        if dp_table.get().is_none() {
            let cap_str = capacity_input.get();
            let w_str = weights_input.get();
            let b_str = benefits_input.get();

            let cap = match cap_str.trim().parse::<usize>() {
                Ok(v) if v > 0 => v,
                _ => {
                    set_error_msg.set(Some("Capacity (m) must be a positive integer.".into()));
                    return;
                }
            };
            let ws = match parse_list(&w_str) {
                Ok(v) if !v.is_empty() => v,
                Err(e) => {
                    set_error_msg.set(Some(format!("Weights: {e}")));
                    return;
                }
                _ => {
                    set_error_msg.set(Some("Enter at least one weight.".into()));
                    return;
                }
            };
            let bs = match parse_list(&b_str) {
                Ok(v) => v,
                Err(e) => {
                    set_error_msg.set(Some(format!("Benefits: {e}")));
                    return;
                }
            };
            if ws.len() != bs.len() {
                set_error_msg.set(Some(format!(
                    "Number of weights ({}) must equal number of benefits ({}).",
                    ws.len(),
                    bs.len()
                )));
                return;
            }

            let table = knapsack_table(cap, &ws, &bs);
            set_capacity.set(cap);
            set_item_weights.set(ws);
            set_item_benefits.set(bs);
            set_dp_table.set(Some(table));
            set_revealed.set(Some(1)); // reveal first cell
            return;
        }

        // Table exists – advance one cell, or wrap around to reset
        match revealed.get() {
            None => {
                // Already fully revealed – reset to step-by-step from scratch
                set_revealed.set(Some(1));
            }
            Some(r) => {
                let next = r + 1;
                if next > total_cells() {
                    set_revealed.set(None); // done – mark all revealed
                } else {
                    set_revealed.set(Some(next));
                }
            }
        }
    };

    // ── Cell visibility predicate ─────────────────────────────────────────────
    // row here is 1-based item row (row 0 is always shown)
    let is_visible = move |row: usize, col: usize, n_cols: usize| -> bool {
        match revealed.get() {
            None => true,
            Some(r) => {
                // linear index in row-major order starting from (row=1, col=0)
                let linear = (row - 1) * n_cols + col;
                linear < r
            }
        }
    };

    // ── View ─────────────────────────────────────────────────────────────────
    view! {
        <div class="page">

            // ── Header ──────────────────────────────────────────────────────
            <header>
                <div class="header-accent"></div>
                <h1>"Knapsack"<span class="accent">"_DP"</span></h1>
                <p class="subtitle">"0 / 1  ·  Dynamic Programming  Visualizer"</p>
            </header>

            // ── Form ────────────────────────────────────────────────────────
            <section class="form-card">
                <div class="field">
                    <label for="cap">"Capacity  "<span class="mono">"m"</span></label>
                    <input
                        id="cap"
                        type="number"
                        min="1"
                        prop:value=move || capacity_input.get()
                        on:input:target=move |ev| set_capacity_input.set(ev.target().value())
                        placeholder="e.g. 6"
                    />
                </div>
                <div class="field">
                    <label for="weights">"Weights  "<span class="mono">"w₁, w₂, …"</span></label>
                    <input
                        id="weights"
                        type="text"
                        prop:value=move || weights_input.get()
                        on:input:target=move |ev| set_weights_input.set(ev.target().value())
                        placeholder="e.g. 2, 3, 4"
                    />
                </div>
                <div class="field">
                    <label for="benefits">"Benefits  "<span class="mono">"b₁, b₂, …"</span></label>
                    <input
                        id="benefits"
                        type="text"
                        prop:value=move || benefits_input.get()
                        on:input:target=move |ev| set_benefits_input.set(ev.target().value())
                        placeholder="e.g. 3, 4, 5"
                    />
                </div>

                <div class="btn-row">
                    <button class="btn btn-solve" on:click=on_solve>"Solve"</button>
                    <button class="btn btn-step"  on:click=on_step>
                        {move || match revealed.get() {
                            None if dp_table.get().is_some() => "↺  Reset steps",
                            _ => "Next step  →",
                        }}
                    </button>
                </div>

                {move || error_msg.get().map(|e| view! {
                    <p class="error">"⚠  "{e}</p>
                })}
            </section>

            // ── Table ────────────────────────────────────────────────────────
            {move || dp_table.get().map(|table| {
                let cap  = capacity.get();
                let ws   = item_weights.get();
                let bs   = item_benefits.get();
                let n    = ws.len();          // number of items
                let n_cols = cap + 1;

                // Current "active" cell for highlighting (last revealed - 1)
                let active_linear: Option<usize> = revealed.get()
                    .and_then(|r| r.checked_sub(1));

                view! {
                    <section class="table-wrap">
                        <table class="dp-table">
                            <thead>
                                <tr>
                                    // top-left corner: "item \ w"
                                    <th class="corner">"item \\ w"</th>
                                    // one column per capacity value 0..=m
                                    {(0..=cap).map(|w| view! {
                                        <th class="w-header">{w}</th>
                                    }).collect_view()}
                                </tr>
                            </thead>
                            <tbody>
                                // Row 0: the "no items" baseline (always fully visible)
                                <tr class="row-base">
                                    <td class="item-header">
                                        <span class="item-badge">"—"</span>
                                        <span class="item-meta">"base"</span>
                                    </td>
                                    {(0..=cap).map(|_| view! {
                                        <td class="cell cell-base">"0"</td>
                                    }).collect_view()}
                                </tr>

                                // Rows 1..=n: one per item
                                {(1..=n).map(|i| {
                                    let wi = ws[i - 1];
                                    let bi = bs[i - 1];
                                    view! {
                                        <tr>
                                            // item header column
                                            <td class="item-header">
                                                <span class="item-badge">{i}</span>
                                                <span class="item-meta">
                                                    "w="<strong>{wi}</strong>
                                                    " b="<strong>{bi}</strong>
                                                </span>
                                            </td>
                                            // data cells
                                            {(0..n_cols).map(|c| {
                                                let linear = (i - 1) * n_cols + c;
                                                let visible = is_visible(i, c, n_cols);
                                                let is_active = active_linear == Some(linear);
                                                let val = table[i][c];

                                                // Did we take the item in this cell?
                                                let took_item = visible
                                                    && wi <= c
                                                    && val == table[i-1][c - wi] + bi
                                                    && val > table[i-1][c];

                                                let cls = if !visible {
                                                    "cell cell-hidden"
                                                } else if is_active {
                                                    "cell cell-active"
                                                } else if took_item {
                                                    "cell cell-took"
                                                } else {
                                                    "cell"
                                                };

                                                view! {
                                                    <td class=cls>
                                                        {if visible { val.to_string() } else { String::new() }}
                                                    </td>
                                                }
                                            }).collect_view()}
                                        </tr>
                                    }
                                }).collect_view()}
                            </tbody>
                        </table>

                        // ── Progress bar ──────────────────────────────────
                        {move || {
                            let total = total_cells();
                            let done  = revealed.get().unwrap_or(total);
                            let pct   = if total > 0 { done * 100 / total } else { 0 };
                            let label = if total == 0 {
                                String::new()
                            } else if done >= total {
                                "✓ Complete".to_string()
                            } else {
                                format!("{} / {} cells", done, total)
                            };
                            view! {
                                <div class="progress-wrap">
                                    <div class="progress-bar" style=format!("width: {}%", pct)></div>
                                    <span class="progress-label">{label}</span>
                                </div>
                            }
                        }}
                    </section>
                }
            })}

        </div>
    }
}
