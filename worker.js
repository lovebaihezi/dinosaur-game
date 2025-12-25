export default {
  async fetch(request, env, ctx) {
    // The assets binding is automatically provided by Cloudflare Workers
    // when using the "assets" configuration in wrangler.jsonc
    return env.ASSETS.fetch(request);
  },
};
