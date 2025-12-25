export default {
  async fetch(request, env, ctx) {
    // The assets binding is automatically provided by Cloudflare Workers
    // when using the "assets" configuration in wrangler.jsonc
    const url = new URL(request.url);
    
    // Handle .meta file requests - return 404 if they don't exist
    // This prevents Bevy from trying to parse the SPA HTML as a meta file
    if (url.pathname.endsWith('.meta')) {
      const response = await env.ASSETS.fetch(request);
      // If the asset returns the SPA fallback (content-type: text/html),
      // it means the .meta file doesn't exist, so return 404
      if (response.headers.get('content-type')?.includes('text/html')) {
        return new Response('Not Found', { status: 404 });
      }
      return response;
    }
    
    return env.ASSETS.fetch(request);
  },
};
