#![cfg(test)]

use super::mock::*;
use super::blogs::*;

use runtime_io::with_externalities;
use srml_support::*;

const ACCOUNT1 : u64 = 1;
const ACCOUNT2 : u64 = 2;

fn blog_slug() -> Vec<u8> {
  b"blog_slug".to_vec()
}

fn blog_ipfs_hash() -> Vec<u8> {
  b"QmRAQB6YaCyidP37UdDnjFY5vQuiBrcqdyoW1CuDgwxkD4".to_vec()
}

fn blog_update(writers: Option<Vec<u64>>, slug: Option<Vec<u8>>, ipfs_hash: Option<Vec<u8>>) -> BlogUpdate<Test> {
  BlogUpdate {
    writers,
    slug,
    ipfs_hash
  }
}

fn post_slug() -> Vec<u8> {
  b"post_slug".to_vec()
}

fn post_ipfs_hash() -> Vec<u8> {
  b"QmRAQB6YaCyidP37UdDnjFY5vQuiBrcqdyoW2CuDgwxkD4".to_vec()
}

fn post_update(blog_id: Option<u32>, slug: Option<Vec<u8>>, ipfs_hash: Option<Vec<u8>>) -> PostUpdate<Test> {
  PostUpdate {
    blog_id,
    slug,
    ipfs_hash
  }
}

fn comment_ipfs_hash() -> Vec<u8> {
  b"QmRAQB6YaCyidP37UdDnjFY5vQuiBrcqdyoW1CuDgwxkD4".to_vec()
}

fn subcomment_ipfs_hash() -> Vec<u8> {
  b"QmYA2fn8cMbVWo4v95RwcwJVyQsNtnEwHerfWR8UNtEwoE".to_vec()
}

fn comment_update(ipfs_hash: Vec<u8>) -> CommentUpdate {
  CommentUpdate {
    ipfs_hash
  }
}

fn reaction_upvote() -> ReactionKind {
  ReactionKind::Upvote
}

fn reaction_downvote() -> ReactionKind {
  ReactionKind::Downvote
}

fn _create_default_blog() -> dispatch::Result {
  _create_blog(None, None, None)
}

fn _create_blog(origin: Option<Origin>, slug: Option<Vec<u8>>, ipfs_hash: Option<Vec<u8>>) -> dispatch::Result {
  Blogs::create_blog(
    origin.unwrap_or(Origin::signed(ACCOUNT1)),
    slug.unwrap_or(self::blog_slug()),
    ipfs_hash.unwrap_or(self::blog_ipfs_hash())
  )
}

fn _update_blog(origin: Option<Origin>, blog_id: Option<u32>, update: Option<BlogUpdate<Test>>) -> dispatch::Result {
  Blogs::update_blog(
    origin.unwrap_or(Origin::signed(ACCOUNT1)),
    blog_id.unwrap_or(1),
    update.unwrap_or(self::blog_update(None, None, None))
  )
}

fn _create_default_post() -> dispatch::Result {
  _create_post(None, None, None, None)
}

fn _create_post(origin: Option<Origin>, blog_id: Option<u32>, slug: Option<Vec<u8>>, ipfs_hash: Option<Vec<u8>>) -> dispatch::Result {
  Blogs::create_post(
    origin.unwrap_or(Origin::signed(ACCOUNT1)),
    blog_id.unwrap_or(1),
    slug.unwrap_or(self::post_slug()),
    ipfs_hash.unwrap_or(self::post_ipfs_hash())
  )
}

fn _update_post(origin: Option<Origin>, post_id: Option<u32>, update: Option<PostUpdate<Test>>) -> dispatch::Result {
  Blogs::update_post(
    origin.unwrap_or(Origin::signed(ACCOUNT1)),
    post_id.unwrap_or(1),
    update.unwrap_or(self::post_update(None, None, None))
  )
}

fn _create_default_comment() -> dispatch::Result {
  _create_comment(None, None, None, None)
}

fn _create_comment(origin: Option<Origin>, post_id: Option<u32>, parent_id: Option<u32>, ipfs_hash: Option<Vec<u8>>) -> dispatch::Result {
  Blogs::create_comment(
    origin.unwrap_or(Origin::signed(ACCOUNT1)),
    post_id.unwrap_or(1),
    parent_id,
    ipfs_hash.unwrap_or(self::comment_ipfs_hash())
  )
}

fn _update_comment(origin: Option<Origin>, comment_id: Option<u32>, update: Option<CommentUpdate>) -> dispatch::Result {
  Blogs::update_comment(
    origin.unwrap_or(Origin::signed(ACCOUNT1)),
    comment_id.unwrap_or(1),
    update.unwrap_or(self::comment_update(self::subcomment_ipfs_hash()))
  )
}

fn _create_default_post_reaction() -> dispatch::Result {
  _create_post_reaction(None, None, None)
}

fn _create_default_comment_reaction() -> dispatch::Result {
  _create_comment_reaction(None, None, None)
}

fn _create_post_reaction(origin: Option<Origin>, post_id: Option<u32>, kind: Option<ReactionKind>) -> dispatch::Result {
  Blogs::create_post_reaction(
    origin.unwrap_or(Origin::signed(ACCOUNT1)),
    post_id.unwrap_or(1),
    kind.unwrap_or(self::reaction_upvote())
  )
}

fn _create_comment_reaction(origin: Option<Origin>, comment_id: Option<u32>, kind: Option<ReactionKind>) -> dispatch::Result {
  Blogs::create_comment_reaction(
    origin.unwrap_or(Origin::signed(ACCOUNT1)),
    comment_id.unwrap_or(1),
    kind.unwrap_or(self::reaction_upvote())
  )
}

fn _update_post_reaction(origin: Option<Origin>, post_id: Option<u32>, reaction_id: u32, kind: Option<ReactionKind>) -> dispatch::Result {
  Blogs::update_post_reaction(
    origin.unwrap_or(Origin::signed(1)),
    post_id.unwrap_or(1),
    reaction_id,
    kind.unwrap_or(self::reaction_upvote())
  )
}

fn _update_comment_reaction(origin: Option<Origin>, comment_id: Option<u32>, reaction_id: u32, kind: Option<ReactionKind>) -> dispatch::Result {
  Blogs::update_comment_reaction(
    origin.unwrap_or(Origin::signed(1)),
    comment_id.unwrap_or(1),
    reaction_id,
    kind.unwrap_or(self::reaction_upvote())
  )
}

fn _delete_post_reaction(origin: Option<Origin>, post_id: Option<u32>, reaction_id: u32) -> dispatch::Result {
  Blogs::delete_post_reaction(
    origin.unwrap_or(Origin::signed(1)),
    post_id.unwrap_or(1),
    reaction_id
  )
}

fn _delete_comment_reaction(origin: Option<Origin>, comment_id: Option<u32>, reaction_id: u32) -> dispatch::Result {
  Blogs::delete_comment_reaction(
    origin.unwrap_or(Origin::signed(1)),
    comment_id.unwrap_or(1),
    reaction_id
  )
}

// Blog tests
#[test]
fn create_blog_should_work() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_blog()); // BlogId 1

    // Check storages
    assert_eq!(Blogs::blog_ids_by_owner(ACCOUNT1), vec![1]);
    assert_eq!(Blogs::blog_id_by_slug(self::blog_slug()), Some(1));
    assert_eq!(Blogs::next_blog_id(), 2);

    // Check whether data stored correctly
    let blog = Blogs::blog_by_id(1).unwrap();

    assert_eq!(blog.created.account, ACCOUNT1);
    assert_eq!(blog.slug, self::blog_slug());
    assert_eq!(blog.ipfs_hash, self::blog_ipfs_hash());
    assert!(blog.writers.is_empty());
    assert_eq!(blog.posts_count, 0);
    assert_eq!(blog.followers_count, 1);
    assert!(blog.edit_history.is_empty());
  });
}

#[test]
fn create_blog_should_fail_short_slug() {
  let slug : Vec<u8> = vec![97; (DEFAULT_SLUG_MIN_LEN - 1) as usize];

  with_externalities(&mut build_ext(), || {
    // Try to catch an error creating a blog with too short slug
    assert_noop!(_create_blog(None, Some(slug), None), MSG_BLOG_SLUG_IS_TOO_SHORT);
  });
}

#[test]
fn create_blog_should_fail_long_slug() {
  let slug : Vec<u8> = vec![97; (DEFAULT_SLUG_MAX_LEN + 1) as usize];

  with_externalities(&mut build_ext(), || {
    // Try to catch an error creating a blog with too long slug
    assert_noop!(_create_blog(None, Some(slug), None), MSG_BLOG_SLUG_IS_TOO_LONG);
  });
}

#[test]
fn create_blog_should_fail_not_unique_slug() {

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_blog()); // BlogId 1
    // Try to catch an error creating a blog with not unique slug
    assert_noop!(_create_default_blog(), MSG_BLOG_SLUG_IS_NOT_UNIQUE);
  });
}

#[test]
fn create_blog_should_fail_invalid_ipfs_hash() {
  let ipfs_hash : Vec<u8> = String::from("QmV9tSDx9UiPeWExXEeH6aoDvmihvx6j").as_bytes().to_vec();

  with_externalities(&mut build_ext(), || {
    // Try to catch an error creating a blog with invalid ipfs_hash
    assert_noop!(_create_blog(None, None, Some(ipfs_hash)), MSG_IPFS_IS_INCORRECT);
  });
}

#[test]
fn update_blog_should_work() {
  let slug : Vec<u8> = String::from("new_slug").as_bytes().to_vec();
  let ipfs_hash : Vec<u8> = String::from("QmRAQB6YaCyidP37UdDnjFY5vQuiBrcqdyoW2CuDgwxkD4").as_bytes().to_vec();

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_blog()); // BlogId 1

    // Blog update with ID 1 should be fine
    assert_ok!(_update_blog(None, None,
      Some(
        self::blog_update(
          None,
          Some(slug.clone()),
          Some(ipfs_hash.clone())
        )
      )
    ));

    // Check whether blog updates correctly
    let blog = Blogs::blog_by_id(1).unwrap();
    assert_eq!(blog.slug, slug);
    assert_eq!(blog.ipfs_hash, ipfs_hash);

    // Check whether history recorded correctly
    assert_eq!(blog.edit_history[0].old_data.writers, None);
    assert_eq!(blog.edit_history[0].old_data.slug, Some(self::blog_slug()));
    assert_eq!(blog.edit_history[0].old_data.ipfs_hash, Some(self::blog_ipfs_hash()));
  });
}

#[test]
fn update_blog_should_fail_nothing_to_update() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_blog()); // BlogId 1
  
    // Try to catch an error updating a blog with no changes
    assert_noop!(_update_blog(None, None, None), MSG_NOTHING_TO_UPDATE_IN_BLOG);
  });
}

#[test]
fn update_blog_should_fail_blog_not_found() {
  let slug : Vec<u8> = String::from("new_slug").as_bytes().to_vec();

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_blog()); // BlogId 1
  
    // Try to catch an error updating a blog with wrong blog ID
    assert_noop!(_update_blog(None, Some(2),
      Some(
        self::blog_update(
          None, 
          Some(slug),
          None
        )
      )
    ), MSG_BLOG_NOT_FOUND);
  });
}

#[test]
fn update_blog_should_fail_not_an_owner() {
  let slug : Vec<u8> = String::from("new_slug").as_bytes().to_vec();

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_blog()); // BlogId 1
  
    // Try to catch an error updating a blog with different account
    assert_noop!(_update_blog(Some(Origin::signed(ACCOUNT2)), None,
      Some(
        self::blog_update(
          None, 
          Some(slug),
          None
        )
      )
    ), MSG_ONLY_BLOG_OWNER_CAN_UPDATE_BLOG);
  });
}

#[test]
fn update_blog_should_fail_short_slug() {
  let slug : Vec<u8> = vec![97; (DEFAULT_SLUG_MIN_LEN - 1) as usize];

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_blog()); // BlogId 1
  
    // Try to catch an error updating a blog with too short slug
    assert_noop!(_update_blog(None, None,
      Some(
        self::blog_update(
          None, 
          Some(slug),
          None
        )
      )
    ), MSG_BLOG_SLUG_IS_TOO_SHORT);
  });
}

#[test]
fn update_blog_should_fail_long_slug() {
  let slug : Vec<u8> = vec![97; (DEFAULT_SLUG_MAX_LEN + 1) as usize];

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_blog()); // BlogId 1
  
    // Try to catch an error updating a blog with too long slug
    assert_noop!(_update_blog(None, None,
      Some(
        self::blog_update(
          None, 
          Some(slug),
          None
        )
      )
    ), MSG_BLOG_SLUG_IS_TOO_LONG);
  });
}

#[test]
fn update_blog_should_fail_not_unique_slug() {
  let slug : Vec<u8> = String::from("unique_slug").as_bytes().to_vec();

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_blog()); // BlogId 1
    
    assert_ok!(_create_blog(
      None,
      Some(slug.clone()),
      None
    )); // BlogId 2 with a custom slug
  
    // Try to catch an error updating a blog on ID 1 with a slug of blog on ID 2
    assert_noop!(_update_blog(None, Some(1),
      Some(
        self::blog_update(
          None, 
          Some(slug),
          None
        )
      )
    ), MSG_BLOG_SLUG_IS_NOT_UNIQUE);
  });
}

#[test]
fn update_blog_should_fail_invalid_ipfs_hash() {
  let ipfs_hash : Vec<u8> = String::from("QmV9tSDx9UiPeWExXEeH6aoDvmihvx6j").as_bytes().to_vec();

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_blog()); // BlogId 1
  
    // Try to catch an error updating a blog with invalid ipfs_hash
    assert_noop!(_update_blog(None, None,
      Some(
        self::blog_update(
          None, 
          None,
          Some(ipfs_hash)
        )
      )
    ), MSG_IPFS_IS_INCORRECT);
  });
}

// Post tests
#[test]
fn create_post_should_work() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_default_post()); // PostId 1

    // Check storages
    assert_eq!(Blogs::post_ids_by_blog_id(1), vec![1]);
    assert_eq!(Blogs::post_id_by_slug(self::post_slug()), Some(1));
    assert_eq!(Blogs::next_post_id(), 2);

    // Check whether data stored correctly
    let post = Blogs::post_by_id(1).unwrap();

    assert_eq!(post.blog_id, 1);
    assert_eq!(post.created.account, ACCOUNT1);
    assert_eq!(post.slug, self::post_slug());
    assert_eq!(post.ipfs_hash, self::post_ipfs_hash());
    assert_eq!(post.comments_count, 0);
    assert_eq!(post.upvotes_count, 0);
    assert_eq!(post.downvotes_count, 0);
    assert!(post.edit_history.is_empty());
  });
}

#[test]
fn create_post_should_fail_short_slug() {
  let slug : Vec<u8> = vec![97; (DEFAULT_SLUG_MIN_LEN - 1) as usize];

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_blog()); // BlogId 1

    // Try to catch an error creating a post with too short slug
    assert_noop!(_create_post(None, None, Some(slug), None), MSG_POST_SLUG_IS_TOO_SHORT);
  });
}

#[test]
fn create_post_should_fail_long_slug() {
  let slug : Vec<u8> = vec![97; (DEFAULT_SLUG_MAX_LEN + 1) as usize];

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_blog()); // BlogId 1

    // Try to catch an error creating a post with too long slug
    assert_noop!(_create_post(None, None, Some(slug), None), MSG_POST_SLUG_IS_TOO_LONG);
  });
}

#[test]
fn create_post_should_fail_not_unique_slug() {

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_default_post()); // PostId 1

    // Try to catch an error creating a post with not unique slug
    assert_noop!(_create_default_post(), MSG_POST_SLUG_IS_NOT_UNIQUE);
  });
}

#[test]
fn create_post_should_fail_invalid_ipfs_hash() {
  let ipfs_hash : Vec<u8> = String::from("QmV9tSDx9UiPeWExXEeH6aoDvmihvx6j").as_bytes().to_vec();

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_blog()); // BlogId 1

    // Try to catch an error creating a post with invalid ipfs_hash
    assert_noop!(_create_post(None, None, None, Some(ipfs_hash)), MSG_IPFS_IS_INCORRECT);
  });
}

#[test]
fn update_post_should_work() {
  let slug : Vec<u8> = String::from("new_slug").as_bytes().to_vec();
  let ipfs_hash : Vec<u8> = String::from("QmRAQB6YaCyidP37UdDnjFY5vQuiBrcqdyoW1CuDgwxkD4").as_bytes().to_vec();

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_default_post()); // PostId 1

    // Post update with ID 1 should be fine
    assert_ok!(_update_post(None, None,
      Some(
        self::post_update(
          None,
          Some(slug.clone()),
          Some(ipfs_hash.clone())
        )
      )
    ));

    
    // Check whether post updates correctly
    let post = Blogs::post_by_id(1).unwrap();
    assert_eq!(post.blog_id, 1);
    assert_eq!(post.slug, slug);
    assert_eq!(post.ipfs_hash, ipfs_hash);

    // Check whether history recorded correctly
    assert_eq!(post.edit_history[0].old_data.blog_id, None);
    assert_eq!(post.edit_history[0].old_data.slug, Some(self::post_slug()));
    assert_eq!(post.edit_history[0].old_data.ipfs_hash, Some(self::post_ipfs_hash()));
  });
}

#[test]
fn update_post_should_fail_nothing_to_update() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_default_post()); // PostId 1
  
    // Try to catch an error updating a post with no changes
    assert_noop!(_update_post(None, None, None), MSG_NOTHING_TO_UPDATE_IN_POST);
  });
}

#[test]
fn update_post_should_fail_post_not_found() {
  let slug : Vec<u8> = String::from("new_slug").as_bytes().to_vec();

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_default_post()); // PostId 1
  
    // Try to catch an error updating a post with wrong post ID
    assert_noop!(_update_post(None, Some(2),
      Some(
        self::post_update(
          None, 
          Some(slug),
          None
        )
      )
    ), MSG_POST_NOT_FOUND);
  });
}

#[test]
fn update_post_should_fail_not_an_owner() {
  let slug : Vec<u8> = String::from("new_slug").as_bytes().to_vec();

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_default_post()); // PostId 1
  
    // Try to catch an error updating a post with different account
    assert_noop!(_update_post(Some(Origin::signed(ACCOUNT2)), None,
      Some(
        self::post_update(
          None, 
          Some(slug),
          None
        )
      )
    ), MSG_ONLY_POST_OWNER_CAN_UPDATE_POST);
  });
}

#[test]
fn update_post_should_fail_short_slug() {
  let slug : Vec<u8> = vec![97; (DEFAULT_SLUG_MIN_LEN - 1) as usize];

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_default_post()); // PostId 1
  
    // Try to catch an error updating a post with too short slug
    assert_noop!(_update_post(None, None,
      Some(
        self::post_update(
          None, 
          Some(slug),
          None
        )
      )
    ), MSG_POST_SLUG_IS_TOO_SHORT);
  });
}

#[test]
fn update_post_should_fail_long_slug() {
  let slug : Vec<u8> = vec![97; (DEFAULT_SLUG_MAX_LEN + 1) as usize];

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_default_post()); // PostId 1
  
    // Try to catch an error updating a post with too long slug
    assert_noop!(_update_post(None, None,
      Some(
        self::post_update(
          None, 
          Some(slug),
          None
        )
      )
    ), MSG_POST_SLUG_IS_TOO_LONG);
  });
}

#[test]
fn update_post_should_fail_not_unique_slug() {
  let slug : Vec<u8> = String::from("unique_slug").as_bytes().to_vec();

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_default_post()); // PostId 1
    
    assert_ok!(_create_post(
      None,
      None,
      Some(slug.clone()),
      None
    )); // PostId 2 with custom slug
  
    // Try to catch an error updating a post on ID 1 with a slug of post on ID 2
    assert_noop!(_update_post(None, Some(1),
      Some(
        self::post_update(
          None, 
          Some(slug),
          None
        )
      )
    ), MSG_POST_SLUG_IS_NOT_UNIQUE);
  });
}

#[test]
fn update_post_should_fail_invalid_ipfs_hash() {
  let ipfs_hash : Vec<u8> = String::from("QmV9tSDx9UiPeWExXEeH6aoDvmihvx6j").as_bytes().to_vec();

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_default_post()); // PostId 1
  
    // Try to catch an error updating a post with invalid ipfs_hash
    assert_noop!(_update_post(None, None,
      Some(
        self::post_update(
          None, 
          None,
          Some(ipfs_hash)
        )
      )
    ), MSG_IPFS_IS_INCORRECT);
  });
}

// Comment tests
#[test]
fn create_comment_should_work() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_default_comment()); // CommentId 1

    // Check storages
    assert_eq!(Blogs::comment_ids_by_post_id(1), vec![1]);
    assert_eq!(Blogs::next_comment_id(), 2);
    assert_eq!(Blogs::post_by_id(1).unwrap().comments_count, 1);

    // Check whether data stored correctly
    let comment = Blogs::comment_by_id(1).unwrap();

    assert_eq!(comment.parent_id, None);
    assert_eq!(comment.post_id, 1);
    assert_eq!(comment.created.account, ACCOUNT1);
    assert_eq!(comment.ipfs_hash, self::comment_ipfs_hash());
    assert_eq!(comment.upvotes_count, 0);
    assert_eq!(comment.downvotes_count, 0);
    assert!(comment.edit_history.is_empty());
  });
}

#[test]
fn create_comment_should_fail_post_not_found() {
  with_externalities(&mut build_ext(), || {
    // Try to catch an error creating a comment with wrong post
    assert_noop!(_create_default_comment(), MSG_POST_NOT_FOUND);
  });
}

#[test]
fn create_comment_should_fail_parent_not_found() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_default_post()); // PostId 1

    // Try to catch an error creating a comment with wrong parent
    assert_noop!(_create_comment(None, None, Some(1), None), MSG_UNKNOWN_PARENT_COMMENT);
  });
}

#[test]
fn create_comment_should_fail_invalid_ipfs_hash() {
  let ipfs_hash : Vec<u8> = String::from("QmV9tSDx9UiPeWExXEeH6aoDvmihvx6j").as_bytes().to_vec();

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_default_post()); // PostId 1

    // Try to catch an error creating a comment with wrong parent
    assert_noop!(_create_comment(None, None, None, Some(ipfs_hash)), MSG_IPFS_IS_INCORRECT);
  });
}

#[test]
fn update_comment_should_work() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_default_comment()); // CommentId 1

    // Post update with ID 1 should be fine
    assert_ok!(_update_comment(
      None,
      None,
      Some(self::comment_update(self::subcomment_ipfs_hash()))
    ));

    // Check whether post updates correctly
    let comment = Blogs::comment_by_id(1).unwrap();
    assert_eq!(comment.ipfs_hash, self::subcomment_ipfs_hash());

    // Check whether history recorded correctly
    assert_eq!(comment.edit_history[0].old_data.ipfs_hash, self::comment_ipfs_hash());
  });
}

#[test]
fn update_comment_should_fail_comment_not_found() {
  with_externalities(&mut build_ext(), || {
    // Try to catch an error updating a comment with wrong CommentId
    assert_noop!(_update_comment(
      None,
      None,
      Some(self::comment_update(self::subcomment_ipfs_hash()))
    ),
    MSG_COMMENT_NOT_FOUND);
  });
}

#[test]
fn update_comment_should_fail_not_an_owner() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_default_comment()); // CommentId 1

    // Try to catch an error updating a comment with wrong Account
    assert_noop!(_update_comment(
      Some(Origin::signed(2)),
      None,
      Some(self::comment_update(self::subcomment_ipfs_hash()))
    ),
    MSG_ONLY_COMMENT_AUTHOR_CAN_UPDATE_COMMENT);
  });
}

#[test]
fn update_comment_should_fail_invalid_ipfs_hash() {
  let ipfs_hash : Vec<u8> = String::from("QmV9tSDx9UiPeWExXEeH6aoDvmihvx6j").as_bytes().to_vec();

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_default_comment()); // CommentId 1

    // Try to catch an error updating a comment with invalid ipfs_hash
    assert_noop!(_update_comment(
      None,
      None,
      Some(self::comment_update(ipfs_hash))
    ),
    MSG_IPFS_IS_INCORRECT);
  });
}

#[test]
fn update_comment_should_fail_ipfs_hash_dont_differ() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_default_comment()); // CommentId 1

    // Try to catch an error updating a comment with the same ipfs_hash
    assert_noop!(_update_comment(
      None,
      None,
      Some(self::comment_update(self::comment_ipfs_hash()))
    ),
    MSG_NEW_COMMENT_HASH_DO_NOT_DIFFER);
  });
}

// Reaction tests
#[test]
fn create_post_reaction_should_work() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_default_post()); // PostId 1

    assert_ok!(_create_default_post_reaction()); // ReactionId 1

    // Check storages
    assert_eq!(Blogs::reaction_ids_by_post_id(1), vec![1]);
    assert_eq!(Blogs::next_reaction_id(), 2);

    // Check post reaction counters
    let post = Blogs::post_by_id(1).unwrap();
    assert_eq!(post.upvotes_count, 1);
    assert_eq!(post.downvotes_count, 0);

    // Check whether data stored correctly
    let reaction = Blogs::reaction_by_id(1).unwrap();
    assert_eq!(reaction.created.account, ACCOUNT1);
    assert_eq!(reaction.kind, self::reaction_upvote());
  });
}

#[test]
fn create_post_reaction_should_fail_already_reacted() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_default_post_reaction()); // ReactionId1 

    // Try to catch an error creating reaction by the same account
    assert_noop!(_create_default_post_reaction(), MSG_ACCOUNT_ALREADY_REACTED_TO_POST);
  });
}

#[test]
fn create_post_reaction_should_fail_post_not_found() {
  with_externalities(&mut build_ext(), || {
    // Try to catch an error creating reaction by the same account
    assert_noop!(_create_default_post_reaction(), MSG_POST_NOT_FOUND);
  });
}

#[test]
fn create_comment_reaction_should_work() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_default_comment()); // CommentId 1
    assert_ok!(_create_default_comment_reaction()); // ReactionId 1

    // Check storages
    assert_eq!(Blogs::reaction_ids_by_comment_id(1), vec![1]);
    assert_eq!(Blogs::next_reaction_id(), 2);

    // Check comment reaction counters
    let comment = Blogs::comment_by_id(1).unwrap();
    assert_eq!(comment.upvotes_count, 1);
    assert_eq!(comment.downvotes_count, 0);

    // Check whether data stored correctly
    let reaction = Blogs::reaction_by_id(1).unwrap();
    assert_eq!(reaction.created.account, ACCOUNT1);
    assert_eq!(reaction.kind, self::reaction_upvote());
  });
}

#[test]
fn create_comment_reaction_should_fail_already_reacted() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_default_comment()); // CommentId 1
    assert_ok!(_create_default_comment_reaction()); // ReactionId 1

    // Try to catch an error creating reaction by the same account
    assert_noop!(_create_default_comment_reaction(), MSG_ACCOUNT_ALREADY_REACTED_TO_COMMENT);
  });
}

#[test]
fn create_comment_reaction_should_fail_comment_not_found() {
  with_externalities(&mut build_ext(), || {
    // Try to catch an error creating reaction by the same account
    assert_noop!(_create_default_comment_reaction(), MSG_COMMENT_NOT_FOUND);
  });
}