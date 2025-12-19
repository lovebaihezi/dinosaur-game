export default {
  async fetch(request: Request, env: Env): Promise<Response> {
    const url = new URL(request.url);
    const path = url.pathname;

    // Root path -> 404 or maybe a landing page?
    // For now, let's assume usage is always /pr/<commit_id>/...
    if (path === "/" || path === "") {
        return new Response("Dino PR Preview System. Go to /pr/<commit_id>/", { status: 200 });
    }

    // Expected format: /pr/<commit_id>/<asset_path>
    // Example: /pr/abcdef123/index.html
    const match = path.match(/^\/pr\/([a-zA-Z0-9]+)\/(.*)/);

    if (!match) {
        return new Response("Invalid Path. Expected /pr/<commit_id>/...", { status: 404 });
    }

    const commitId = match[1];
    let assetPath = match[2];

    // Handle root of the PR (e.g. /pr/abcdef123/ -> index.html)
    if (assetPath === "" || assetPath.endsWith("/")) {
        assetPath += "index.html";
    }

    const r2Key = `pr/${commitId}/${assetPath}`;

    // Fetch from R2
    const object = await env.ASSETS_BUCKET.get(r2Key);

    if (!object) {
        // Try index.html for SPA routing if strictly needed, but Trunk usually outputs relative paths.
        // For now, simple 404.
        return new Response(`Not Found: ${r2Key}`, { status: 404 });
    }

    const headers = new Headers();
    object.writeHttpMetadata(headers);
    headers.set("etag", object.httpEtag);

    return new Response(object.body, {
        headers,
    });
  }
}

interface Env {
  ASSETS_BUCKET: R2Bucket;
}
