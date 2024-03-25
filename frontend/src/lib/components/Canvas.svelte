<script lang="ts">
    import Konva from 'konva';
    import { onMount, createEventDispatcher } from "svelte";
    import { writable } from 'svelte/store';
    import type { DrawTool } from '../images/types';
    
    export let imageData: ImageData;
    export let drawTool: DrawTool;

    $: if (drawTool) {
        console.log('drawTool changed:', drawTool);
    }

    let stage: Konva.Stage;
    let layer: Konva.Layer;
    let background: Konva.Image;

    // Drawing state
    let isDrawing = false;
    let currentRect: Konva.Rect | null = null;
    let startPoint = { x: 0, y: 0 };

    const stageWidth = writable(imageData.width);
    const stageHeight = writable(imageData.height);

    let isCtrlPanning = false;
    let lastPosX: number = 0;
    let lastPosY: number = 0;

    const dispatch = createEventDispatcher();

    onMount(() => {
        stage = new Konva.Stage({
            container: "container",
            width: $stageWidth,
            height: $stageHeight
        });

        layer = new Konva.Layer({
            imageSmoothingEnabled: false
        });
        stage.add(layer);

        drawImageData(imageData);
        enableInteraction();
        adjustStageSize();

        window.addEventListener('resize', adjustStageSize);

        return () => {
            window.removeEventListener('resize', adjustStageSize);
        }
    });

    $: imageData, drawImageData(imageData);

    function adjustStageSize() {
        const container = document.getElementById('container');
        if (!container) return;

        const scale = Math.min(container.offsetWidth / imageData.width, container.offsetHeight / imageData.height);
        stageWidth.set(imageData.width * scale);
        stageHeight.set(imageData.height * scale);

        stage.width($stageWidth);
        stage.height($stageHeight);
        stage.scale({ x: scale, y: scale });
        stage.draw();
    }

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
        stage.on('wheel', handleWheel);
    }

    function handleMouseDown(e: Konva.KonvaEventObject<MouseEvent>) {
        const startDrawing = (e: Konva.KonvaEventObject<MouseEvent>) => {
            isDrawing = true;
            startPoint = stage.getPointerPosition();
            currentRect = new Konva.Rect({
                x: startPoint.x,
                y: startPoint.y,
                width: 0,
                height: 0,
                stroke: 'red',
                strokeWidth: 2,
            });
            layer.add(currentRect);
        }

        const startCtrlPanning = (e: Konva.KonvaEventObject<MouseEvent>) => {
            isCtrlPanning = true;
            lastPosX = e.evt.clientX;
            lastPosY = e.evt.clientY;
            stage.container().style.cursor = "grabbing";
        }
        
        if (e.evt.ctrlKey) {
            startCtrlPanning(e)
        }
        else {
            startDrawing(e)
        }
    }

    function handleMouseMove(e: Konva.KonvaEventObject<MouseEvent>) {
        const pointerPosition = stage.getPointerPosition();

        if (pointerPosition) {
            dispatch('mouseMove', {
                x: pointerPosition.x,
                y: pointerPosition.y
            });
        }

        const draw = (e: Konva.KonvaEventObject<MouseEvent>) => {
            if (currentRect && pointerPosition) {
                currentRect.width(pointerPosition.x - startPoint.x);
                currentRect.height(pointerPosition.y - startPoint.y);
                layer.batchDraw();
            }
        }

        const ctrlPan = (e: Konva.KonvaEventObject<MouseEvent>) => {
            const dx = e.evt.clientX - lastPosX;
            const dy = e.evt.clientY - lastPosY;
            stage.x(stage.x() + dx);
            stage.y(stage.y() + dy);
            stage.batchDraw();

            lastPosX = e.evt.clientX;
            lastPosY = e.evt.clientY;
        }

        if (isDrawing && !isCtrlPanning) {
            draw(e);
        } else if (isCtrlPanning) {
            ctrlPan(e);
        }
    }

    function handleMouseUp(e: Konva.KonvaEventObject<MouseEvent>) {
        isCtrlPanning = false;
        stage.container().style.cursor = 'default';
        if (isDrawing) {
            isDrawing = false;
            currentRect = null;
        }
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

<div id="container" class="w-full h-full" />