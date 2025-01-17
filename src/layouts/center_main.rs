use crate::models::Tag;
use crate::models::Window;
use crate::models::Workspace;

/// Layout which splits the workspace into three columns.
/// Gives first window all of the center column.
/// Gives second window all of the left column.
/// Divides the right column among all the other windows.
///
/// 1 window
/// ```text
/// +-----------------------+
/// |                       |
/// |                       |
/// |           1           |
/// |                       |
/// |                       |
/// +-----------------------+
/// ```
/// 2 windows
/// ```text
/// +-----------+-----------+
/// |           |           |
/// |           |           |
/// |      2    |     1     |
/// |           |           |
/// |           |           |
/// +-----------+-----------+
/// ```
/// 3 windows
/// ```text
/// +-----+-----------+-----+
/// |     |           |     |
/// |     |           |     |
/// |  2  |     1     |  3  |
/// |     |           |     |
/// |     |           |     |
/// +-----+-----------+-----+
/// ```
/// 4 windows
/// ```text
/// +-----+-----------+-----+
/// |     |           |  3  |
/// |     |           |     |
/// |  2  |     1     +-----+
/// |     |           |  4  |
/// |     |           |     |
/// +-----+-----------+-----+
/// ```
/// 5 windows
/// ```text
/// +-----+-----------+-----+
/// |     |           |  3  |
/// |     |           +-----+
/// |  2  |     1     |  4  |
/// |     |           +-----+
/// |     |           |  5  |
/// +-----+-----------+-----+
/// ```
pub fn update(workspace: &Workspace, windows: &mut Vec<&mut Window>, tags: &mut Vec<Tag>) {
    let window_count = windows.len();

    if window_count == 0 {
        return;
    }

    let primary_width = match window_count {
        1 => workspace.width() as i32,
        _ => (workspace.width() as f32 / 2.0).floor() as i32,
    };

    let secondary_width = match window_count {
        1 => 0,
        2 => (workspace.width() as f32 / 2.0).floor() as i32,
        _ => (workspace.width() as f32 / 4.0).floor() as i32,
    };

    let (primary_x, secondary_x, stack_x) = match window_count {
        1 => (0, 0, 0),
        2 => {
            let (px, sx);
            if workspace.flipped_horizontal(tags) {
                px = workspace.x();
                sx = workspace.x() + primary_width;
            } else {
                px = (workspace.width() as f32 / 2.0).floor() as i32;
                sx = workspace.x();
            }
            (px, sx, 0)
        }
        _ => {
            let px = (workspace.width() as f32 / 4.0).floor() as i32;
            let (sx, stx);
            if workspace.flipped_horizontal(tags) {
                sx = workspace.x() + primary_width + secondary_width;
                stx = workspace.x();
            } else {
                sx = workspace.x();
                stx = workspace.x() + primary_width + secondary_width;
            }
            (px, sx, stx)
        }
    };

    let mut iter = windows.iter_mut();

    // build the primary window
    {
        if let Some(first) = iter.next() {
            first.set_height(workspace.height());
            first.set_width(primary_width);
            first.set_x(workspace.x() + primary_x);
            first.set_y(workspace.y());
        }
    }

    // build the second window
    {
        if let Some(second) = iter.next() {
            second.set_height(workspace.height());
            second.set_width(secondary_width);
            second.set_x(secondary_x);
            second.set_y(workspace.y());
        }
    }

    // stack all the others
    if window_count > 2 {
        let height_f = workspace.height() as f32 / (window_count - 2) as f32;
        let height = height_f.floor() as i32;
        let mut y = 0;

        for w in iter {
            w.set_height(height);
            w.set_width(secondary_width);
            w.set_x(stack_x);
            w.set_y(workspace.y() + y);
            y += height;
        }
    }
}
