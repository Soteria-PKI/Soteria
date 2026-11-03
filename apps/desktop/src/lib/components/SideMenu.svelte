<script lang="ts">
import { page } from "$app/state";
import { appRoutes } from "../routes.js";
let collapsed = $state(false);
function currentPage(route: string): Boolean {
  console.log(page.url.pathname);
  return route === page.url.pathname;
}
</script>

<main>
  <nav class="sidebar sidebar-expanded" aria-label="Main Navigation">
    <div class="sidebar-header">
      <div class="brand-logo">
        <!-- this is the brand logo -->
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
      <button class="btn-collapse" aria-label="Toggle rail">&laquo;</button>
    </div>

    <ul class="nav-list">
      {#each appRoutes as route}
        <li>
          <a
            href={route.path}
            class={[
              "nav-item",
              {
                "nav-item active": currentPage(route.path),
              },
            ]}
          >
            <span class="nav-label">{route.name}</span>
            <span class="nav-badge">142</span>
          </a>
        </li>
      {/each}
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
