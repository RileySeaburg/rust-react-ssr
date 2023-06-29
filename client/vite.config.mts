import { defineConfig } from 'npm:vite@^4.0.4'
import react from 'npm:@vitejs/plugin-react@^3.0.1'

import 'npm:react@^18.2.0'
import 'npm:react-dom@^18.2.0/client'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  build: {
    target: 'es2015',
    outDir: 'dist/client', // Output directory for the client bundle
    ssr: 'src/entry-server.tsx', // Entry file for the server bundle
    minify: false,
    rollupOptions: {
      input: {
        server: 'src/entry-server.tsx', // Entry file for the server bundle (Rollup input)
      },
      external: ['react', 'react-dom'], // External dependencies for the server bundle
      output: {
        format: 'es', // Output format for the server bundle (Rollup output)
        exports: 'auto', // Automatically handle exports
      },
    },
  },
})
