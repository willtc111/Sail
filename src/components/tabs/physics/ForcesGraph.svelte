<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  import Chart from 'chart.js/auto';

  let width = 300;
  let height = 300;

  let canvas: HTMLCanvasElement;
  let ctx: CanvasRenderingContext2D | null;
  $: if (canvas) {
    ctx = canvas.getContext('2d');
    drawCoefficients(ctx!);
  }

  async function drawCoefficients(ctx: CanvasRenderingContext2D) {
    let labels = [...Array(180).keys()];
    let coefficients = await invoke('debug_coefficients') as number[][];
    let cls = coefficients[0];
    let cds = coefficients[1];

    let myLineChart = new Chart(ctx, {
        type: 'line',
        data: {
            labels: labels,
            datasets: [
                {
                    label: 'Lift',
                    data: cls,
                    borderColor: 'blue',
                    borderWidth: 1,
                    fill: false,
                    pointStyle: false,
                },
                {
                    label: 'Drag',
                    data: cds,
                    borderColor: 'red',
                    borderWidth: 1,
                    fill: false,
                    pointStyle: false,
                }
            ]
        },
        options: {
            responsive: true,
            
        }
    });
  }

</script>

<div>
  <canvas
    bind:this={canvas}
    {width}
    {height}
    class="w-full h-full rounded-container-token"
  />
</div>