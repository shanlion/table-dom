<template>
  <div :style="{ display: 'flex', position: 'sticky', top: '0', backgroundColor: '#f5f5f5', zIndex: 1 }">
    <div v-for="(_, i) in tableCols" :key="i" :style="{ width: colWidth + 'px', padding: '4px', boxSizing: 'border-box' }">
      Column {{ i + 1 }}
    </div>
  </div>
  <div ref="scrollDomRef" :style="{ height: '300px', overflowY: 'auto', position: 'relative', backgroundColor: '#eee' }">
    <div :style="{ height: tableHeight + 'px', position: 'relative' }">
      <div
        v-for="(row, i) in visibleRows"
        :key="i"
        :style="{
          position: 'absolute',
          top: (startRow + i) * rowHeight + 'px',
          left: 0,
          display: 'flex'
        }"
      >
        <div
          v-for="(_, j) in tableCols"
          :key="j"
          :style="{
            width: colWidth + 'px',
            padding: '4px',
            boxSizing: 'border-box',
            borderBottom: '1px solid #ccc',
            borderRight: '1px solid #ccc'
          }"
        >
          Row {{ startRow + i + 1 }}, Column {{ j + 1 }}
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted, watch } from 'vue'

const tableCols = Array.from({ length: 30 })
const tableData = Array.from({ length: 1000 })

const rowHeight = 50
const colWidth = 100
const overscan = 5

const scrollTop = ref(0)
const visibleRows = ref([])
const startRow = ref(0)
const scrollDomRef = ref(null)

const tableHeight = tableData.length * rowHeight

const updateVisibleRows = () => {
  const viewHeight = scrollDomRef.value?.clientHeight || 0
  const scrolledTop = scrollTop.value

  const visibleCount = Math.ceil(viewHeight / rowHeight)
  startRow.value = Math.max(0, Math.floor(scrolledTop / rowHeight) - overscan)
  const endRow = Math.min(tableData.length - 1, startRow.value + visibleCount + overscan * 2)

  visibleRows.value = tableData.slice(startRow.value, endRow + 1)
}

const handleScroll = () => {
  scrollTop.value = scrollDomRef.value.scrollTop
  updateVisibleRows()
}

onMounted(() => {
  scrollDomRef.value.addEventListener('scroll', handleScroll)
  updateVisibleRows()
})

onUnmounted(() => {
  scrollDomRef.value?.removeEventListener('scroll', handleScroll)
})
</script>

<style scoped>
div {
  font-family: Arial, sans-serif;
  font-size: 14px;
}
</style>
