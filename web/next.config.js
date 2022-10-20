/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  swcMinify: true,

  // docker
  output: 'standalone'
}

module.exports = nextConfig
