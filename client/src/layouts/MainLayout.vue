<template>
  <q-layout view="hHh lpR fFf">>
    <q-header flat class="q-px-auto" >
      <q-toolbar>
        <q-space></q-space>

        <q-toolbar-title>
          <a
            href="/#"
           >
            Jonathan Sindorf
          </a>
        </q-toolbar-title>

        <div>Quasar v{{ $q.version }}</div>
        <q-space></q-space>
      </q-toolbar>
    </q-header>

    <q-page-container>
      <router-view />
    </q-page-container>
    <q-footer flat class="flex flex-center">
      Clock
    </q-footer>
  </q-layout>
</template>

<script>
import { defineComponent } from 'vue'

export default defineComponent({
  name: 'MainLayout',

  setup () {
    return {
    }
  }
})
</script>
