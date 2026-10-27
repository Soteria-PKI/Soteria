<script lang="ts">
    let routes = import.meta.glob('../../routes/*/+page.svelte');
</script>

<main>
    <nav class="sidebar sidebar-expanded" aria-label="Main Navigation">
        <div class="sidebar-header">
            <div class="brand-logo">
                <div class="brand-icon">
                    <svg
                        viewBox="0 0 24 24"
                        width="18"
                        height="18"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                    >
                        <rect x="3" y="3" width="7" height="7" rx="1" />
                        <rect x="14" y="3" width="7" height="7" rx="1" />
                        <rect x="14" y="14" width="7" height="7" rx="1" />
                        <rect x="3" y="14" width="7" height="7" rx="1" />
                    </svg>
                </div>
                <div class="brand-info">
                    <div class="brand-title">Soteria</div>
                    <div class="brand-subtitle">Management system</div>
                </div>
            </div>
            <button class="btn-collapse" aria-label="Toggle rail"
                >&laquo;</button
            >
        </div>

        <ul class="nav-list">
            <li>
                <a href="#inventory" class="nav-item active">
                    <span class="nav-label">Inventory</span>
                    <span class="nav-badge">142</span>
                </a>
            </li>
            <li>
                <a href="#purchase-orders" class="nav-item">
                    <span class="nav-label">Purchase Orders</span>
                    <span class="nav-badge">18</span>
                </a>
            </li>
            <li>
                <a href="#service-orders" class="nav-item">
                    <span class="nav-label">Service Orders</span>
                    <span class="nav-badge">9</span>
                </a>
            </li>
            <li>
                <a href="#sale" class="nav-item">
                    <span class="nav-label">Sale</span>
                </a>
            </li>
        </ul>

        <div class="sidebar-footer">
            <div class="user-chip">
                <div class="user-avatar">AM</div>
                <div class="user-details">
                    <div class="user-name">Alex Mercer</div>
                    <div class="user-role">Operations Lead</div>
                </div>
            </div>
            <button class="btn-logout" title="Sign out">&rarr;</button>
        </div>
    </nav>
</main>
