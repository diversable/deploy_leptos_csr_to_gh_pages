<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

# Deploy Leptos CSR Apps to Github Pages

Step 1:
Go to your repo's settings, and click on "Pages". In the "Build and deployment" section of the page, change the "source" to "Github Actions".

Step 2:
Copy the `.github/workflows/gh-pages-deploy.yml` file into your own repo.

Step 3:
Commit to your main branch, and watch the magic happen.