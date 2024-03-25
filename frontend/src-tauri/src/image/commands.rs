use super::{error::{ImageManagerError, ImageViewError}, file::{open_tiff, save_tiff_stack}, statistics::{calculate_histogram, Histogram}, view::{ImageView, ImageViewController, ImageViewId, ImageViewSettings, TsImageView}};
use std::{path::PathBuf, sync::{Arc, Mutex}};
use tauri::{AppHandle, State};

#[tauri::command]
pub fn save_as_tiff(
    controller: State<Mutex<ImageViewController>>,
    path: PathBuf,
    id: ImageViewId,
) -> Result<(), ImageViewError> {
    save_tiff_stack(controller.lock().unwrap().get_view(id)?, path);
    Ok(())
}

// // #[tauri::command]
// // pub fn save_image_as_bitmap(
// //     manager: State<Mutex<ImageManager>>,
// //     path: PathBuf,
// //     id: ImageId,
// // ) {
// //     manager.lock().unwrap().save_image(id, path, image::ImageFormat::Bmp);
// // }


#[tauri::command]
pub fn open_image(
    app: AppHandle,
    controller: State<Mutex<ImageViewController>>,
    path: PathBuf,
) -> Result<TsImageView, ImageManagerError> {
    let stack = open_tiff(path)?;
    let view = ImageView::new(app, ImageViewSettings::default(), Arc::new(stack));
    let ts_view = view.get_ts_view();
    let mut controller = controller.lock().unwrap();
    controller.add_view(view);
    Ok(ts_view)
}

// pub fn get_image_thumbnail(
//     manager: State<Mutex<ImageManager>>,
//     stack_id: ImageStackId,
//     image_id: ImageId,
//     width: u32,
//     height: u32
// ) -> Result<Image, ImageManagerError> {
//     let manager = manager.lock().unwrap();
//     let stack = manager.get_stack(stack_id)?;
//     let image = stack.get_image(image_id)?;
//     Ok(image.thumbnail(width, height)?)
// }

#[tauri::command]
pub fn list_all_views(
    app: AppHandle,
    controller: State<Mutex<ImageViewController>>
) -> Vec<TsImageView> {
    controller.lock().unwrap().list_all_views()
}

#[tauri::command]
pub fn set_view_slice(
    controller: State<Mutex<ImageViewController>>,
    id: ImageViewId,
    slice: usize
) -> Result<(), ImageViewError> {
    let mut controller = controller.lock().unwrap();
    controller.get_view_mut(id)?.set_slice(slice)
}

#[tauri::command]
pub fn update_view_settings(
    controller: State<Mutex<ImageViewController>>,
    id: ImageViewId,
    settings: ImageViewSettings
) -> Result<(), ImageViewError> {
    controller.lock().unwrap().update_view_settings(id, settings)
}

#[tauri::command]
pub fn get_pixel_value(
    controller: State<Mutex<ImageViewController>>,
    id: ImageViewId,
    x: u32,
    y: u32
) -> Result<u32, ImageViewError> {
    controller.lock().unwrap().get_view(id)?.get_pixel_value(x, y)
}

// #[tauri::command]
// pub fn histogram(
//     manager: State<Mutex<ImageManager>>,
//     stack_id: ImageStackId,
//     image_id: ImageId,
//     roi: ROI,
//     num_bins: u32
// ) -> Result<Histogram, ImageManagerError> {
//     let manager = manager.lock().unwrap();
//     let stack = manager.get_stack(stack_id)?;    
//     let image = stack.get_image(image_id)?;
//     let iter = image.iter(roi);
//     Ok(calculate_histogram(iter, 65536, num_bins))
// }

// #[tauri::command]
// pub fn std(
//     manager: State<Mutex<ImageManager>>,
//     stack_id: ImageStackId,
//     image_id: ImageId,
//     roi: Option<ROI>
// ) {
    
// }