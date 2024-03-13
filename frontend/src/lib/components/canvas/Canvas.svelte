<script lang="ts">
    import Konva from 'konva';
    import { onMount } from "svelte";

    export let imageData: ImageData;

    let stage: Konva.Stage;
    let layer: Konva.Layer;
    let background: Konva.Image;
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
        stage.add(layer);

        // Initial draw or setup if needed
        if (imageData) {
            drawImageData(imageData);
        }
    });

    $: imageData, drawImageData(imageData);

    function drawImageData(imageData: ImageData) {
        if (!stage || !imageData) return;

        const canvas = document.createElement('canvas');
        canvas.width = imageData.width;
        canvas.height = imageData.height;
        const ctx = canvas.getContext('2d');
        if (ctx) {
            ctx.putImageData(imageData, 0, 0);
            const image = new Image();
            image.onload = () => {
                if (!background) {
                    background = new Konva.Image({
                        image: image,
                        x: 0,
                        y: 0,
                        width: stageWidth,
                        height: stageHeight,
                    });
                    layer.add(background);
                } else {
                    background.image(image);
                }
                layer.batchDraw();
            };
            image.src = canvas.toDataURL();
        }
    }

    function enableInteraction() {
        stage.on('mousedown', handleMouseDown);
        stage.on('mousemove', handleMouseMove);
        stage.on('mouseup', handleMouseUp);
        stage.on('mouseleave', handleMouseLeave);
        stage.on('handlewheel', handleWheel);
    }

    function disableInteraction() {
        stage.off('mousedown', handleMouseDown);
        stage.off('mousemove', handleMouseMove);
        stage.off('mouseup', handleMouseUp);
        stage.off('mouseleave', handleMouseLeave);
        stage.off('handlewheel', handleWheel);
    }

    function handleMouseDown(e: Konva.KonvaEventObject<MouseEvent>) {
        if (e.evt.ctrlKey) {
            isCtrlPanning = true;
            lastPosX = e.evt.clientX;
            lastPosY = e.evt.clientY;
            stage.container().style.cursor = "grabbing";
        }
    }

    function handleMouseMove(e: Konva.KonvaEventObject<MouseEvent>) {
        if (isCtrlPanning) {
            const dx = e.evt.clientX - lastPosX;
            const dy = e.evt.clientY - lastPosY;
            stage.x(stage.x() + dx);
            stage.y(stage.y() + dy);
            stage.batchDraw();

            lastPosX = e.evt.clientX;
            lastPosY = e.evt.clientY;
        }
    }

    function handleMouseUp(e: Konva.KonvaEventObject<MouseEvent>) {
        isCtrlPanning = false;
        stage.container().style.cursor = 'default';
    }

    function handleMouseLeave(e: Konva.KonvaEventObject<MouseEvent>) {
        isCtrlPanning = false;
        stage.container().style.cursor = 'default';
    }

    function handleWheel(e: Konva.KonvaEventObject<WheelEvent>) {
        e.evt.preventDefault();
        const oldScale = stage.scaleX();
        const scaleBy = 1.1;
        const pointer = stage.getPointerPosition();

        if (!pointer) return; 

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
    }
</script>

<div id="container" class="w-full h-full bg-gray-300" />