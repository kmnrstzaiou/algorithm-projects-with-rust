const NUM_DISKS: usize = 64;

fn main() {
    // Make three posts with NUM_DISKS entries, all set to 0.
    let mut posts = [[0; NUM_DISKS]; 3];

    // Put the disks on the first post in order, smallest first (on top).
    for i in 0..NUM_DISKS {
        posts[0][i] = i + 1;
    }

    // Draw the initial setup.
    draw_posts(&posts);

    // Move the disks.
    move_disks(&mut posts, NUM_DISKS, 0, 1, 2);
    println!("Ok");
}

fn move_disks(
    posts: &mut [[usize; NUM_DISKS]; 3],
    num_to_move: usize,
    from_post: usize,
    to_post: usize,
    temp_post: usize,
) {
    if num_to_move > 1 {
        move_disks(posts, num_to_move - 1, from_post, temp_post, to_post);
        move_disk(posts, from_post, to_post);
        draw_posts(posts);
        move_disks(posts, num_to_move - 1, temp_post, to_post, from_post);
    } else {
        move_disk(posts, from_post, to_post);
        draw_posts(posts);
    }
}

fn draw_posts(posts: &[[usize; NUM_DISKS]; 3]) {
    for i in 0..NUM_DISKS {
        for post in posts {
            print!("{} ", post[i]);
        }
        println!();
    }
    println!("-----");
}

fn move_disk(posts: &mut [[usize; NUM_DISKS]; 3], from_post: usize, to_post: usize) {
    for i in 0..NUM_DISKS {
        if posts[from_post][i] != 0 {
            for j in (0..NUM_DISKS).rev() {
                if posts[to_post][j] == 0 {
                    posts[to_post][j] = posts[from_post][i];
                    posts[from_post][i] = 0;
                    return;
                }
            }
        }
    }
}
