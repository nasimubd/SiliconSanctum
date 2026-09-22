use ratatui::layout::Rect;
use sanctum_runtime::tui::render::regions;
#[test]fn keeps_regions_in_bounds(){let area=Rect::new(0,0,20,8);let regions=regions(area);assert!(regions.body.bottom()<=area.bottom());assert!(regions.footer.bottom()<=area.bottom());}
