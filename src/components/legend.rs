use leptos::prelude::*;

#[component]
pub fn KnapsackLegend() -> impl IntoView {
    view! {
        <section class="legend-card">
            <h2 class="legend-title">"Legend"</h2>
            <div class="legend-items">
                <div class="legend-item">
                    <div class="legend-cell cell-took">"4"</div>
                    <span>"Item was "<strong>"taken"</strong>" (better value including this item)"</span>
                </div>
                <div class="legend-item">
                    <div class="legend-cell">"3"</div>
                    <span>"Item was "<strong>"skipped"</strong>" (inherited value from row above)"</span>
                </div>
                <div class="legend-item">
                    <div class="legend-cell cell-backtrack">"7★"</div>
                    <span>"Part of the "<strong>"backtracking path"</strong>" — these cells trace back the optimal solution"</span>
                </div>
            </div>
        </section>
    }
}
