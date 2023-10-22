import { RouteRecordRaw } from 'vue-router';

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    component: () => import('layouts/MainLayout.vue'),
    children: [
      {
        name: 'analyzer',
        path: '/analyzer',
        component: () => import('pages/AnalyzerPage.vue'),
      },
      {
        name: 'editor',
        path: '/editor',
        component: () => import('pages/EditorPage.vue'),
      },
      { path: '/', redirect: '/editor' },
    ],
  },

  // Always leave this as last one,
  // but you can also remove it
  {
    path: '/:catchAll(.*)*',
    component: () => import('pages/ErrorNotFound.vue'),
  },
];

export default routes;
