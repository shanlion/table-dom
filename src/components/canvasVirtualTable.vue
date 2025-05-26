
<template>
  <canvas ref="canvasRef"></canvas>
</template>

<script setup>
import { onMounted, onUnmounted, ref } from 'vue'

const canvasRef = ref(null)
const rowHeight = 20
const tableCols = Array.from({length: 30}, (_, i) => i + 1)
const tableData = Array.from({length: 1000}, (_, i) => i + 1)

const generateRowData = (rowIndex) => {
  return tableCols.reduce((obj, col, idx) => {
    obj[`col_${col}`] = `Row ${rowIndex}Col ${col}`
    return obj
  }, { id: rowIndex })
}

const data = tableData.map((_, i) => generateRowData(i))

const render = () => {
  const canvas = canvasRef.value
  const ctx = canvas.getContext('2d')
  const scrollTop = window.scrollY

  canvas.width = window.innerWidth
  canvas.height = window.innerHeight

  const startRow = Math.floor(scrollTop / rowHeight)
  const visibleRowCount = Math.ceil(canvas.height / rowHeight)

  ctx.clearRect(0, 0, canvas.width, canvas.height)
  ctx.textBaseline = 'middle'

  const colWidth = canvas.width / tableCols.length

  for (let i = 0; i < visibleRowCount; i++) {
    const rowIndex = startRow + i
    const row = data[rowIndex]
    if (!row) continue
    const y = i * rowHeight

    ctx.strokeRect(0, y, canvas.width, rowHeight)
    
    tableCols.forEach((col, colIdx) => {
      const x = colIdx * colWidth + 5
      ctx.fillText(row[`col_${col}`], x, y + rowHeight / 2)
    })
  }
}

const onScroll = () => {
  render()
}

onMounted(() => {
  render()
  window.addEventListener('scroll', onScroll)
  document.body.style.height = `${data.length * rowHeight}px`
})

onUnmounted(() => {
  window.removeEventListener('scroll', onScroll)
})
</script>

<style scoped>
canvas {
  position: fixed;
  top: 0;
  left: 0;
  z-index: 1;
}
body {
  margin: 0;
}
</style>
