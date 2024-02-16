<script lang="ts">
    import Konva from "konva";
    import { onMount } from "svelte";

    let stage: Konva.Stage;
    let layer: Konva.Layer;
    let stageWidth: number = 300;
    let stageHeight: number = 300;

    let isCtrlPanning = false;
    let lastPosX: number = 0;
    let lastPosY: number = 0;

    onMount(() => {
        stage = new Konva.Stage({
            container: "container",
            width: stageWidth,
            height: stageHeight
        });
        
        layer = new Konva.Layer();
        const background = new Konva.Rect({
            x: 0, y: 0, fill: "green", width: stageWidth, height: stageHeight
        });
        layer.add(background);
        stage.add(layer);

        stage.on('mousedown', (e) => {
            if (e.evt.ctrlKey) {
                isCtrlPanning = true;
                lastPosX = e.evt.clientX;
                lastPosY = e.evt.clientY;
                stage.container().style.cursor = "grabbing";
            }
        })

        stage.on('mousemove', (e: Konva.KonvaEventObject<MouseEvent>) => {
            if (isCtrlPanning) {
                const dx = e.evt.clientX - lastPosX;
                const dy = e.evt.clientY - lastPosY;
                stage.x(stage.x() + dx);
                stage.y(stage.y() + dy);
                stage.batchDraw();

                lastPosX = e.evt.clientX;
                lastPosY = e.evt.clientY;
            }
        });

        stage.on('mouseup', (e: Konva.KonvaEventObject<MouseEvent>) => {
            isCtrlPanning = false;
            stage.container().style.cursor = 'default'; // Reset cursor
        });

        stage.on('mouseleave', (e: Konva.KonvaEventObject<MouseEvent>) => {
            isCtrlPanning = false;
            stage.container().style.cursor = 'default'; // Reset cursor
        });

        stage.on('wheel', (e: Konva.KonvaEventObject<WheelEvent>) => {
            e.evt.preventDefault();
            const oldScale = stage.scaleX();
            const scaleBy = 1.1;
            const pointer = stage.getPointerPosition();

            if (!pointer) return; // Exit if no pointer position is available

            const mousePointTo = {
                x: (pointer.x - stage.x()) / oldScale,
                y: (pointer.y - stage.y()) / oldScale,
            };

            const newScale = e.evt.deltaY > 0 ? oldScale * scaleBy : oldScale / scaleBy;

            stage.scale({ x: newScale, y: newScale });

            const newPos = {
                x: pointer.x - mousePointTo.x * newScale,
                y: pointer.y - mousePointTo.y * newScale,
            };
            stage.position(newPos);
            stage.batchDraw();
        });
    });
</script>

<div id="container" class="w-full h-full bg-red-800" />