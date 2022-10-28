/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  swcMinify: true,
  experimental: { appDir: true, }
  // docker
  // output: 'standalone'
}

module.exports = nextConfig
