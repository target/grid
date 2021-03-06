/**
 * Copyright 2018-2020 Cargill Incorporated
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

const { createProxyMiddleware } = require('http-proxy-middleware');

// This file is used as a proxy middleware to avoid CORS errors for the
// development server when connecting to the splinter and grid daemons in
// grid/examples/splinter/docker-compose.yaml

module.exports = function(app) {
  app.use(
    '/splinterd',
    createProxyMiddleware({
      target: 'http://localhost:8085',
      changeOrigin: true,
      pathRewrite: {
        '^/splinterd': '/'
      }
    })
  );
  app.use(
    '/gridd',
    createProxyMiddleware({
      target: 'http://localhost:8080',
      changeOrigin: true,
      pathRewrite: {
        '^/gridd': '/'
      }
    })
  );
};
