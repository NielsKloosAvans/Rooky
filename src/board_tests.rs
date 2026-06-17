use super::*;

#[test]
fn empty_board_has_no_piece_on_e4() {
    let board = Board::empty();

    let e4 = Square::new(4, 3).unwrap();

    assert!(board.is_empty(e4));
}

#[test]
fn set_piece_stores_piece_on_e4() {
    let mut board = Board::empty();

    let e4 = Square::new(4, 3).unwrap();

    let white_queen = Piece::new(Color::White, PieceKind::Queen);

    board.set_piece(e4, white_queen);

    assert_eq!(board.piece_at(e4), Some(white_queen));
}

#[test]
fn remove_piece_on_e4() {
    let mut board = Board::empty();

    let e4 = Square::new(4, 3).unwrap();

    let white_queen = Piece::new(Color::White, PieceKind::Queen);

    board.set_piece(e4, white_queen);

    assert_eq!(board.remove_piece(e4), Some(white_queen));
    assert_eq!(board.piece_at(e4), None);
}

#[test]
fn remove_piece_on_empty_square() {
    let mut board = Board::empty();

    let e4 = Square::new(4, 3).unwrap();

    assert_eq!(board.remove_piece(e4), None);
}

#[test]
fn starting_position_has_expected_key_squares() {
    let board = Board::starting_position();

    let expected_positions = [
        (3, 0, Some(Piece::new(Color::White, PieceKind::Queen))),
        (4, 0, Some(Piece::new(Color::White, PieceKind::King))),
        (3, 7, Some(Piece::new(Color::Black, PieceKind::Queen))),
        (4, 7, Some(Piece::new(Color::Black, PieceKind::King))),
        (4, 1, Some(Piece::new(Color::White, PieceKind::Pawn))),
        (7, 1, Some(Piece::new(Color::White, PieceKind::Pawn))),
        (4, 6, Some(Piece::new(Color::Black, PieceKind::Pawn))),
        (7, 6, Some(Piece::new(Color::Black, PieceKind::Pawn))),
        (4, 3, None),
    ];

    for (file, rank, expected_piece) in expected_positions {
        let square = Square::new(file, rank).unwrap();

        assert_eq!(board.piece_at(square), expected_piece);
    }
}

#[test]
fn board_is_not_empty_after_setting_a_piece() {
    let mut board = Board::empty();

    let e4 = Square::new(4, 3).unwrap();
    let white_queen = Piece::new(Color::White, PieceKind::Queen);

    board.set_piece(e4, white_queen);

    assert!(!board.is_empty(e4))
}

#[test]
fn make_move_moves_piece_from_e2_to_e4() {
    let mut board = Board::starting_position();
    let e2 = Square::new(4, 1).unwrap();
    let e4 = Square::new(4, 3).unwrap();

    let e2_to_e4 = ChessMove::new(e2, e4);

    board.make_move(e2_to_e4);

    assert!(board.is_empty(e2));
    assert_eq!(
        board.piece_at(e4),
        Some(Piece::new(Color::White, PieceKind::Pawn))
    );
}

#[test]
fn make_move_to_empty_square_returns_none() {
    let mut board = Board::starting_position();
    let e2 = Square::new(4, 1).unwrap();
    let e4 = Square::new(4, 3).unwrap();

    let e2_to_e4 = ChessMove::new(e2, e4);

    let captured_piece = board.make_move(e2_to_e4);

    assert_eq!(captured_piece, None);
}

#[test]
fn make_move_to_occupied_square_returns_captured_piece() {
    let mut board = Board::empty();
    let e4 = Square::new(4, 3).unwrap();
    let e5 = Square::new(4, 4).unwrap();
    let white_queen = Piece::new(Color::White, PieceKind::Queen);
    let black_pawn = Piece::new(Color::Black, PieceKind::Pawn);

    board.set_piece(e4, white_queen);
    board.set_piece(e5, black_pawn);

    let captured_piece = board.make_move(ChessMove::new(e4, e5));

    assert_eq!(captured_piece, Some(black_pawn));
    assert!(board.is_empty(e4));
    assert_eq!(board.piece_at(e5), Some(white_queen));
}

#[test]
fn make_move_from_empty_square_does_nothing() {
    let mut board = Board::empty();

    let e2 = Square::new(4, 1).unwrap();
    let e4 = Square::new(4, 3).unwrap();

    let e2_to_e4 = ChessMove::new(e2, e4);

    board.make_move(e2_to_e4);

    assert!(board.is_empty(e2));
    assert!(board.is_empty(e4));
}

#[test]
fn fen_piece_placement_starting_position_matches_starting_position() {
    let board = Board::from_fen_piece_placement("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");

    assert_eq!(board, Ok(Board::starting_position()));
}

#[test]
fn fen_piece_placement_empty_board_has_no_piece_on_e4() {
    let board = Board::from_fen_piece_placement("8/8/8/8/8/8/8/8").unwrap();

    let e4 = Square::new(4, 3).unwrap();

    assert!(board.is_empty(e4));
}

#[test]
fn fen_piece_placement_rejects_too_few_ranks() {
    let board = Board::from_fen_piece_placement("rnbqkbnr/pppppppp/8/8/8/8/8");

    assert_eq!(board, Err(FenError::WrongRankCount));
}

#[test]
fn fen_piece_placement_rejects_too_many_ranks() {
    let board = Board::from_fen_piece_placement("rnbqkbnr/pppppppp/8/8/8/8/8/8/8");

    assert_eq!(board, Err(FenError::WrongRankCount));
}

#[test]
fn fen_piece_placement_rejects_rank_with_too_few_files() {
    let board = Board::from_fen_piece_placement("7/8/8/8/8/8/8/8");

    assert_eq!(board, Err(FenError::WrongFileCount));
}

#[test]
fn fen_piece_placement_rejects_rank_with_too_many_files() {
    let board = Board::from_fen_piece_placement("9/8/8/8/8/8/8/8");

    assert_eq!(board, Err(FenError::WrongFileCount));
}

#[test]
fn fen_piece_placement_rejects_unknown_piece_letter() {
    let board = Board::from_fen_piece_placement("x7/8/8/8/8/8/8/8");

    assert_eq!(board, Err(FenError::InvalidPiece));
}

#[test]
fn empty_square_has_no_piece_moves() {
    let board = Board::empty();
    let e4 = Square::new(4, 3).unwrap();

    assert!(board.piece_moves_from(e4).is_empty());
}

#[test]
fn piece_moves_from_dispatches_to_pawn_moves() {
    let mut board = Board::empty();
    let e2 = Square::new(4, 1).unwrap();

    board.set_piece(e2, Piece::new(Color::White, PieceKind::Pawn));

    assert_eq!(board.piece_moves_from(e2), board.pawn_moves_from(e2));
}

#[test]
fn piece_moves_from_dispatches_to_knight_moves() {
    let mut board = Board::empty();
    let e4 = Square::new(4, 3).unwrap();

    board.set_piece(e4, Piece::new(Color::White, PieceKind::Knight));

    assert_eq!(board.piece_moves_from(e4), board.knight_moves_from(e4));
}

#[test]
fn piece_moves_from_dispatches_to_bishop_moves() {
    let mut board = Board::empty();
    let e4 = Square::new(4, 3).unwrap();

    board.set_piece(e4, Piece::new(Color::White, PieceKind::Bishop));

    assert_eq!(board.piece_moves_from(e4), board.bishop_moves_from(e4));
}

#[test]
fn piece_moves_from_dispatches_to_rook_moves() {
    let mut board = Board::empty();
    let e4 = Square::new(4, 3).unwrap();

    board.set_piece(e4, Piece::new(Color::White, PieceKind::Rook));

    assert_eq!(board.piece_moves_from(e4), board.rook_moves_from(e4));
}

#[test]
fn piece_moves_from_dispatches_to_queen_moves() {
    let mut board = Board::empty();
    let e4 = Square::new(4, 3).unwrap();

    board.set_piece(e4, Piece::new(Color::White, PieceKind::Queen));

    assert_eq!(board.piece_moves_from(e4), board.queen_moves_from(e4));
}

#[test]
fn piece_moves_from_dispatches_to_king_moves() {
    let mut board = Board::empty();
    let e4 = Square::new(4, 3).unwrap();

    board.set_piece(e4, Piece::new(Color::White, PieceKind::King));

    assert_eq!(board.piece_moves_from(e4), board.king_moves_from(e4));
}

#[test]
fn empty_board_has_no_pseudo_legal_moves_for_white() {
    let board = Board::empty();

    assert!(board.pseudo_legal_moves_for(Color::White).is_empty());
}

#[test]
fn pseudo_legal_moves_for_white_ignores_black_pieces() {
    let mut board = Board::empty();
    let b1 = Square::new(1, 0).unwrap();
    let g8 = Square::new(6, 7).unwrap();

    board.set_piece(b1, Piece::new(Color::White, PieceKind::Knight));
    board.set_piece(g8, Piece::new(Color::Black, PieceKind::Knight));

    assert_eq!(
        board.pseudo_legal_moves_for(Color::White),
        board.knight_moves_from(b1)
    );
}

#[test]
fn starting_position_has_20_white_pseudo_legal_moves() {
    let board = Board::starting_position();

    assert_eq!(board.pseudo_legal_moves_for(Color::White).len(), 20);
}

#[test]
fn starting_position_has_20_black_pseudo_legal_moves() {
    let board = Board::starting_position();

    assert_eq!(board.pseudo_legal_moves_for(Color::Black).len(), 20);
}

#[test]
fn empty_board_has_no_white_king_square() {
    let board = Board::empty();

    assert_eq!(board.king_square(Color::White), None);
}

#[test]
fn starting_position_white_king_is_on_e1() {
    let board = Board::starting_position();
    let e1 = Square::new(4, 0).unwrap();

    assert_eq!(board.king_square(Color::White), Some(e1));
}

#[test]
fn starting_position_black_king_is_on_e8() {
    let board = Board::starting_position();
    let e8 = Square::new(4, 7).unwrap();

    assert_eq!(board.king_square(Color::Black), Some(e8));
}

#[test]
fn king_square_ignores_enemy_king() {
    let mut board = Board::empty();
    let e1 = Square::new(4, 0).unwrap();

    board.set_piece(e1, Piece::new(Color::White, PieceKind::King));

    assert_eq!(board.king_square(Color::Black), None);
}

#[test]
fn empty_board_has_no_attacked_square() {
    let board = Board::empty();
    let e4 = Square::new(4, 3).unwrap();

    assert!(!board.is_square_attacked(e4, Color::White));
}

#[test]
fn knight_attacks_target_square() {
    let mut board = Board::empty();
    let e4 = Square::new(4, 3).unwrap();
    let f6 = Square::new(5, 5).unwrap();

    board.set_piece(e4, Piece::new(Color::White, PieceKind::Knight));

    assert!(board.is_square_attacked(f6, Color::White));
}

#[test]
fn attack_detection_ignores_wrong_color() {
    let mut board = Board::empty();
    let e4 = Square::new(4, 3).unwrap();
    let f6 = Square::new(5, 5).unwrap();

    board.set_piece(e4, Piece::new(Color::White, PieceKind::Knight));

    assert!(!board.is_square_attacked(f6, Color::Black));
}

#[test]
fn white_pawn_attacks_diagonal_squares_but_not_forward() {
    let mut board = Board::empty();
    let e4 = Square::new(4, 3).unwrap();
    let d5 = Square::new(3, 4).unwrap();
    let e5 = Square::new(4, 4).unwrap();
    let f5 = Square::new(5, 4).unwrap();

    board.set_piece(e4, Piece::new(Color::White, PieceKind::Pawn));

    assert!(board.is_square_attacked(d5, Color::White));
    assert!(!board.is_square_attacked(e5, Color::White));
    assert!(board.is_square_attacked(f5, Color::White));
}

#[test]
fn black_pawn_attacks_diagonal_squares_down_the_board() {
    let mut board = Board::empty();
    let e5 = Square::new(4, 4).unwrap();
    let d4 = Square::new(3, 3).unwrap();
    let f4 = Square::new(5, 3).unwrap();

    board.set_piece(e5, Piece::new(Color::Black, PieceKind::Pawn));

    assert!(board.is_square_attacked(d4, Color::Black));
    assert!(board.is_square_attacked(f4, Color::Black));
}

#[test]
fn side_with_no_king_is_not_in_check() {
    let board = Board::empty();

    assert!(!board.is_in_check(Color::White));
}

#[test]
fn white_king_alone_is_not_in_check() {
    let mut board = Board::empty();
    let e1 = Square::new(4, 0).unwrap();

    board.set_piece(e1, Piece::new(Color::White, PieceKind::King));

    assert!(!board.is_in_check(Color::White));
}

#[test]
fn white_king_is_in_check_from_black_rook_on_same_file() {
    let mut board = Board::empty();
    let e1 = Square::new(4, 0).unwrap();
    let e8 = Square::new(4, 7).unwrap();

    board.set_piece(e1, Piece::new(Color::White, PieceKind::King));
    board.set_piece(e8, Piece::new(Color::Black, PieceKind::Rook));

    assert!(board.is_in_check(Color::White));
}

#[test]
fn white_king_is_not_in_check_when_piece_blocks_rook() {
    let mut board = Board::empty();
    let e1 = Square::new(4, 0).unwrap();
    let e4 = Square::new(4, 3).unwrap();
    let e8 = Square::new(4, 7).unwrap();

    board.set_piece(e1, Piece::new(Color::White, PieceKind::King));
    board.set_piece(e4, Piece::new(Color::White, PieceKind::Bishop));
    board.set_piece(e8, Piece::new(Color::Black, PieceKind::Rook));

    assert!(!board.is_in_check(Color::White));
}

#[test]
fn white_king_is_in_check_from_black_knight() {
    let mut board = Board::empty();
    let e1 = Square::new(4, 0).unwrap();
    let f3 = Square::new(5, 2).unwrap();

    board.set_piece(e1, Piece::new(Color::White, PieceKind::King));
    board.set_piece(f3, Piece::new(Color::Black, PieceKind::Knight));

    assert!(board.is_in_check(Color::White));
}

#[test]
fn black_king_is_in_check_from_white_pawn() {
    let mut board = Board::empty();
    let d4 = Square::new(3, 3).unwrap();
    let e5 = Square::new(4, 4).unwrap();

    board.set_piece(d4, Piece::new(Color::White, PieceKind::Pawn));
    board.set_piece(e5, Piece::new(Color::Black, PieceKind::King));

    assert!(board.is_in_check(Color::Black));
}

#[test]
fn starting_position_has_20_white_legal_moves() {
    let board = Board::starting_position();

    assert_eq!(board.legal_moves_for(Color::White).len(), 20);
}

#[test]
fn legal_moves_do_not_allow_king_to_move_into_check() {
    let mut board = Board::empty();
    let e1 = Square::new(4, 0).unwrap();
    let e2 = Square::new(4, 1).unwrap();
    let e8 = Square::new(4, 7).unwrap();

    board.set_piece(e1, Piece::new(Color::White, PieceKind::King));
    board.set_piece(e8, Piece::new(Color::Black, PieceKind::Rook));

    assert!(
        !board
            .legal_moves_for(Color::White)
            .contains(&ChessMove::new(e1, e2))
    );
}

#[test]
fn legal_moves_do_not_allow_pinned_piece_to_expose_king() {
    let mut board = Board::empty();
    let e1 = Square::new(4, 0).unwrap();
    let e2 = Square::new(4, 1).unwrap();
    let d2 = Square::new(3, 1).unwrap();
    let e8 = Square::new(4, 7).unwrap();

    board.set_piece(e1, Piece::new(Color::White, PieceKind::King));
    board.set_piece(e2, Piece::new(Color::White, PieceKind::Rook));
    board.set_piece(e8, Piece::new(Color::Black, PieceKind::Rook));

    assert!(
        !board
            .legal_moves_for(Color::White)
            .contains(&ChessMove::new(e2, d2))
    );
}

#[test]
fn legal_moves_allow_pinned_piece_to_move_along_pin_line() {
    let mut board = Board::empty();
    let e1 = Square::new(4, 0).unwrap();
    let e2 = Square::new(4, 1).unwrap();
    let e3 = Square::new(4, 2).unwrap();
    let e8 = Square::new(4, 7).unwrap();

    board.set_piece(e1, Piece::new(Color::White, PieceKind::King));
    board.set_piece(e2, Piece::new(Color::White, PieceKind::Rook));
    board.set_piece(e8, Piece::new(Color::Black, PieceKind::Rook));

    assert!(
        board
            .legal_moves_for(Color::White)
            .contains(&ChessMove::new(e2, e3))
    );
}

#[test]
fn legal_moves_allow_king_to_capture_attacker_when_destination_is_safe() {
    let mut board = Board::empty();
    let e1 = Square::new(4, 0).unwrap();
    let e2 = Square::new(4, 1).unwrap();

    board.set_piece(e1, Piece::new(Color::White, PieceKind::King));
    board.set_piece(e2, Piece::new(Color::Black, PieceKind::Rook));

    assert!(
        board
            .legal_moves_for(Color::White)
            .contains(&ChessMove::new(e1, e2))
    );
}

#[test]
fn legal_moves_do_not_allow_king_to_capture_protected_attacker() {
    let mut board = Board::empty();
    let e1 = Square::new(4, 0).unwrap();
    let e2 = Square::new(4, 1).unwrap();
    let b5 = Square::new(1, 4).unwrap();

    board.set_piece(e1, Piece::new(Color::White, PieceKind::King));
    board.set_piece(e2, Piece::new(Color::Black, PieceKind::Rook));
    board.set_piece(b5, Piece::new(Color::Black, PieceKind::Bishop));

    assert!(
        !board
            .legal_moves_for(Color::White)
            .contains(&ChessMove::new(e1, e2))
    );
}

#[test]
fn white_is_checkmated_when_in_check_with_no_legal_moves() {
    let mut board = Board::empty();
    let h1 = Square::new(7, 0).unwrap();
    let g2 = Square::new(6, 1).unwrap();
    let f3 = Square::new(5, 2).unwrap();

    board.set_piece(h1, Piece::new(Color::White, PieceKind::King));
    board.set_piece(g2, Piece::new(Color::Black, PieceKind::Queen));
    board.set_piece(f3, Piece::new(Color::Black, PieceKind::King));

    assert!(board.is_checkmate(Color::White));
}

#[test]
fn white_is_not_checkmated_when_it_can_capture_attacker() {
    let mut board = Board::empty();
    let h1 = Square::new(7, 0).unwrap();
    let g2 = Square::new(6, 1).unwrap();
    let a8 = Square::new(0, 7).unwrap();

    board.set_piece(h1, Piece::new(Color::White, PieceKind::King));
    board.set_piece(g2, Piece::new(Color::Black, PieceKind::Queen));
    board.set_piece(a8, Piece::new(Color::Black, PieceKind::King));

    assert!(!board.is_checkmate(Color::White));
}

#[test]
fn starting_position_is_not_checkmate() {
    let board = Board::starting_position();

    assert!(!board.is_checkmate(Color::White));
}

#[test]
fn white_is_stalemated_when_not_in_check_and_has_no_legal_moves() {
    let mut board = Board::empty();
    let h1 = Square::new(7, 0).unwrap();
    let f2 = Square::new(5, 1).unwrap();
    let g3 = Square::new(6, 2).unwrap();

    board.set_piece(h1, Piece::new(Color::White, PieceKind::King));
    board.set_piece(f2, Piece::new(Color::Black, PieceKind::Queen));
    board.set_piece(g3, Piece::new(Color::Black, PieceKind::King));

    assert!(board.is_stalemate(Color::White));
}

#[test]
fn white_is_not_stalemated_when_in_check() {
    let mut board = Board::empty();
    let h1 = Square::new(7, 0).unwrap();
    let g2 = Square::new(6, 1).unwrap();
    let f3 = Square::new(5, 2).unwrap();

    board.set_piece(h1, Piece::new(Color::White, PieceKind::King));
    board.set_piece(g2, Piece::new(Color::Black, PieceKind::Queen));
    board.set_piece(f3, Piece::new(Color::Black, PieceKind::King));

    assert!(!board.is_stalemate(Color::White));
}

#[test]
fn starting_position_is_not_stalemate() {
    let board = Board::starting_position();

    assert!(!board.is_stalemate(Color::White));
}

#[test]
fn white_pawn_on_e2_can_move_to_e3() {
    let mut board = Board::empty();
    let e2 = Square::new(4, 1).unwrap();
    let e3 = Square::new(4, 2).unwrap();

    board.set_piece(e2, Piece::new(Color::White, PieceKind::Pawn));

    assert!(board.pawn_moves_from(e2).contains(&ChessMove::new(e2, e3)));
}

#[test]
fn black_pawn_on_e7_can_move_to_e6() {
    let mut board = Board::empty();
    let e7 = Square::new(4, 6).unwrap();
    let e6 = Square::new(4, 5).unwrap();

    board.set_piece(e7, Piece::new(Color::Black, PieceKind::Pawn));

    assert!(board.pawn_moves_from(e7).contains(&ChessMove::new(e7, e6)));
}

#[test]
fn white_pawn_blocked_on_e2_has_no_moves() {
    let mut board = Board::empty();
    let e2 = Square::new(4, 1).unwrap();
    let e3 = Square::new(4, 2).unwrap();

    board.set_piece(e2, Piece::new(Color::White, PieceKind::Pawn));
    board.set_piece(e3, Piece::new(Color::Black, PieceKind::Knight));

    assert!(board.pawn_moves_from(e2).is_empty());
}

#[test]
fn black_pawn_blocked_on_e7_has_no_moves() {
    let mut board = Board::empty();
    let e7 = Square::new(4, 6).unwrap();
    let e6 = Square::new(4, 5).unwrap();

    board.set_piece(e7, Piece::new(Color::Black, PieceKind::Pawn));
    board.set_piece(e6, Piece::new(Color::White, PieceKind::Knight));

    assert!(board.pawn_moves_from(e7).is_empty());
}

#[test]
fn empty_square_has_no_pawn_moves() {
    let board = Board::empty();
    let e4 = Square::new(4, 3).unwrap();

    assert!(board.pawn_moves_from(e4).is_empty());
}

#[test]
fn queen_square_has_no_pawn_moves() {
    let mut board = Board::empty();
    let e4 = Square::new(4, 3).unwrap();

    board.set_piece(e4, Piece::new(Color::White, PieceKind::Queen));

    assert!(board.pawn_moves_from(e4).is_empty());
}

#[test]
fn white_pawn_on_e2_can_move_two_squares_to_e4() {
    let mut board = Board::empty();
    let e2 = Square::new(4, 1).unwrap();
    let e3 = Square::new(4, 2).unwrap();
    let e4 = Square::new(4, 3).unwrap();

    board.set_piece(e2, Piece::new(Color::White, PieceKind::Pawn));

    assert_eq!(
        board.pawn_moves_from(e2),
        vec![ChessMove::new(e2, e3), ChessMove::new(e2, e4)]
    );
}

#[test]
fn black_pawn_on_e7_can_move_two_squares_to_e5() {
    let mut board = Board::empty();
    let e7 = Square::new(4, 6).unwrap();
    let e6 = Square::new(4, 5).unwrap();
    let e5 = Square::new(4, 4).unwrap();

    board.set_piece(e7, Piece::new(Color::Black, PieceKind::Pawn));

    assert_eq!(
        board.pawn_moves_from(e7),
        vec![ChessMove::new(e7, e6), ChessMove::new(e7, e5)]
    );
}

#[test]
fn white_pawn_not_on_starting_rank_cannot_move_two_squares() {
    let mut board = Board::empty();
    let e3 = Square::new(4, 2).unwrap();
    let e4 = Square::new(4, 3).unwrap();

    board.set_piece(e3, Piece::new(Color::White, PieceKind::Pawn));

    assert_eq!(board.pawn_moves_from(e3), vec![ChessMove::new(e3, e4)]);
}

#[test]
fn black_pawn_not_on_starting_rank_cannot_move_two_squares() {
    let mut board = Board::empty();
    let e6 = Square::new(4, 5).unwrap();
    let e5 = Square::new(4, 4).unwrap();

    board.set_piece(e6, Piece::new(Color::Black, PieceKind::Pawn));

    assert_eq!(board.pawn_moves_from(e6), vec![ChessMove::new(e6, e5)]);
}

#[test]
fn white_pawn_cannot_double_move_if_e3_is_blocked() {
    let mut board = Board::empty();
    let e2 = Square::new(4, 1).unwrap();
    let e3 = Square::new(4, 2).unwrap();

    board.set_piece(e2, Piece::new(Color::White, PieceKind::Pawn));
    board.set_piece(e3, Piece::new(Color::Black, PieceKind::Knight));

    assert!(board.pawn_moves_from(e2).is_empty());
}

#[test]
fn white_pawn_cannot_double_move_if_e4_is_blocked() {
    let mut board = Board::empty();
    let e2 = Square::new(4, 1).unwrap();
    let e3 = Square::new(4, 2).unwrap();
    let e4 = Square::new(4, 3).unwrap();

    board.set_piece(e2, Piece::new(Color::White, PieceKind::Pawn));
    board.set_piece(e4, Piece::new(Color::Black, PieceKind::Knight));

    assert_eq!(board.pawn_moves_from(e2), vec![ChessMove::new(e2, e3)]);
}

#[test]
fn white_pawn_on_e4_can_capture_black_piece_on_d5() {
    let mut board = Board::empty();
    let e4 = Square::new(4, 3).unwrap();
    let d5 = Square::new(3, 4).unwrap();

    board.set_piece(e4, Piece::new(Color::White, PieceKind::Pawn));
    board.set_piece(d5, Piece::new(Color::Black, PieceKind::Knight));

    assert!(board.pawn_moves_from(e4).contains(&ChessMove::new(e4, d5)));
}

#[test]
fn white_pawn_on_e4_can_capture_black_piece_on_f5() {
    let mut board = Board::empty();
    let e4 = Square::new(4, 3).unwrap();
    let f5 = Square::new(5, 4).unwrap();

    board.set_piece(e4, Piece::new(Color::White, PieceKind::Pawn));
    board.set_piece(f5, Piece::new(Color::Black, PieceKind::Knight));

    assert!(board.pawn_moves_from(e4).contains(&ChessMove::new(e4, f5)));
}

#[test]
fn black_pawn_on_e5_can_capture_white_piece_on_d4() {
    let mut board = Board::empty();
    let e5 = Square::new(4, 4).unwrap();
    let d4 = Square::new(3, 3).unwrap();

    board.set_piece(e5, Piece::new(Color::Black, PieceKind::Pawn));
    board.set_piece(d4, Piece::new(Color::White, PieceKind::Knight));

    assert!(board.pawn_moves_from(e5).contains(&ChessMove::new(e5, d4)));
}

#[test]
fn black_pawn_on_e5_can_capture_white_piece_on_f4() {
    let mut board = Board::empty();
    let e5 = Square::new(4, 4).unwrap();
    let f4 = Square::new(5, 3).unwrap();

    board.set_piece(e5, Piece::new(Color::Black, PieceKind::Pawn));
    board.set_piece(f4, Piece::new(Color::White, PieceKind::Knight));

    assert!(board.pawn_moves_from(e5).contains(&ChessMove::new(e5, f4)));
}

#[test]
fn white_pawn_cannot_capture_empty_diagonal_square() {
    let mut board = Board::empty();
    let e4 = Square::new(4, 3).unwrap();
    let e5 = Square::new(4, 4).unwrap();
    let d5 = Square::new(3, 4).unwrap();

    board.set_piece(e4, Piece::new(Color::White, PieceKind::Pawn));

    let moves = board.pawn_moves_from(e4);

    assert_eq!(moves, vec![ChessMove::new(e4, e5)]);
    assert!(!moves.contains(&ChessMove::new(e4, d5)));
}

#[test]
fn white_pawn_cannot_capture_own_piece() {
    let mut board = Board::empty();
    let e4 = Square::new(4, 3).unwrap();
    let e5 = Square::new(4, 4).unwrap();
    let d5 = Square::new(3, 4).unwrap();

    board.set_piece(e4, Piece::new(Color::White, PieceKind::Pawn));
    board.set_piece(d5, Piece::new(Color::White, PieceKind::Knight));

    let moves = board.pawn_moves_from(e4);

    assert_eq!(moves, vec![ChessMove::new(e4, e5)]);
    assert!(!moves.contains(&ChessMove::new(e4, d5)));
}

#[test]
fn pawn_on_a_file_does_not_wrap_left() {
    let mut board = Board::empty();
    let a4 = Square::new(0, 3).unwrap();
    let a5 = Square::new(0, 4).unwrap();
    let h5 = Square::new(7, 4).unwrap();

    board.set_piece(a4, Piece::new(Color::White, PieceKind::Pawn));
    board.set_piece(h5, Piece::new(Color::Black, PieceKind::Knight));

    let moves = board.pawn_moves_from(a4);

    assert_eq!(moves, vec![ChessMove::new(a4, a5)]);
    assert!(!moves.contains(&ChessMove::new(a4, h5)));
}

#[test]
fn pawn_on_h_file_does_not_wrap_right() {
    let mut board = Board::empty();
    let h4 = Square::new(7, 3).unwrap();
    let h5 = Square::new(7, 4).unwrap();
    let a5 = Square::new(0, 4).unwrap();

    board.set_piece(h4, Piece::new(Color::White, PieceKind::Pawn));
    board.set_piece(a5, Piece::new(Color::Black, PieceKind::Knight));

    let moves = board.pawn_moves_from(h4);

    assert_eq!(moves, vec![ChessMove::new(h4, h5)]);
    assert!(!moves.contains(&ChessMove::new(h4, a5)));
}

#[test]
fn white_knight_on_e4_can_move_to_8_squares() {
    let mut board = Board::empty();
    let e4 = Square::new(4, 3).unwrap();

    board.set_piece(e4, Piece::new(Color::White, PieceKind::Knight));

    let expected_moves = vec![
        ChessMove::new(e4, Square::new(5, 5).unwrap()),
        ChessMove::new(e4, Square::new(6, 4).unwrap()),
        ChessMove::new(e4, Square::new(6, 2).unwrap()),
        ChessMove::new(e4, Square::new(5, 1).unwrap()),
        ChessMove::new(e4, Square::new(3, 1).unwrap()),
        ChessMove::new(e4, Square::new(2, 2).unwrap()),
        ChessMove::new(e4, Square::new(2, 4).unwrap()),
        ChessMove::new(e4, Square::new(3, 5).unwrap()),
    ];

    assert_eq!(board.knight_moves_from(e4), expected_moves);
}

#[test]
fn knight_on_a1_does_not_wrap_off_board() {
    let mut board = Board::empty();
    let a1 = Square::new(0, 0).unwrap();

    board.set_piece(a1, Piece::new(Color::White, PieceKind::Knight));

    let expected_moves = vec![
        ChessMove::new(a1, Square::new(1, 2).unwrap()),
        ChessMove::new(a1, Square::new(2, 1).unwrap()),
    ];

    assert_eq!(board.knight_moves_from(a1), expected_moves);
}

#[test]
fn knight_can_capture_enemy_piece() {
    let mut board = Board::empty();
    let e4 = Square::new(4, 3).unwrap();
    let f6 = Square::new(5, 5).unwrap();

    board.set_piece(e4, Piece::new(Color::White, PieceKind::Knight));
    board.set_piece(f6, Piece::new(Color::Black, PieceKind::Pawn));

    assert!(
        board
            .knight_moves_from(e4)
            .contains(&ChessMove::new(e4, f6))
    );
}

#[test]
fn knight_cannot_capture_own_piece() {
    let mut board = Board::empty();
    let e4 = Square::new(4, 3).unwrap();
    let f6 = Square::new(5, 5).unwrap();

    board.set_piece(e4, Piece::new(Color::White, PieceKind::Knight));
    board.set_piece(f6, Piece::new(Color::White, PieceKind::Pawn));

    assert!(
        !board
            .knight_moves_from(e4)
            .contains(&ChessMove::new(e4, f6))
    );
}

#[test]
fn empty_square_has_no_knight_moves() {
    let board = Board::empty();
    let e4 = Square::new(4, 3).unwrap();

    assert!(board.knight_moves_from(e4).is_empty());
}

#[test]
fn pawn_square_has_no_knight_moves() {
    let mut board = Board::empty();
    let e4 = Square::new(4, 3).unwrap();

    board.set_piece(e4, Piece::new(Color::White, PieceKind::Pawn));

    assert!(board.knight_moves_from(e4).is_empty());
}

#[test]
fn white_king_on_e4_can_move_to_8_squares() {
    let mut board = Board::empty();
    let e4 = Square::new(4, 3).unwrap();

    board.set_piece(e4, Piece::new(Color::White, PieceKind::King));

    let expected_moves = vec![
        ChessMove::new(e4, Square::new(4, 4).unwrap()),
        ChessMove::new(e4, Square::new(5, 4).unwrap()),
        ChessMove::new(e4, Square::new(5, 3).unwrap()),
        ChessMove::new(e4, Square::new(5, 2).unwrap()),
        ChessMove::new(e4, Square::new(4, 2).unwrap()),
        ChessMove::new(e4, Square::new(3, 2).unwrap()),
        ChessMove::new(e4, Square::new(3, 3).unwrap()),
        ChessMove::new(e4, Square::new(3, 4).unwrap()),
    ];

    assert_eq!(board.king_moves_from(e4), expected_moves);
}

#[test]
fn king_on_a1_does_not_wrap_off_board() {
    let mut board = Board::empty();
    let a1 = Square::new(0, 0).unwrap();

    board.set_piece(a1, Piece::new(Color::White, PieceKind::King));

    let expected_moves = vec![
        ChessMove::new(a1, Square::new(0, 1).unwrap()),
        ChessMove::new(a1, Square::new(1, 1).unwrap()),
        ChessMove::new(a1, Square::new(1, 0).unwrap()),
    ];

    assert_eq!(board.king_moves_from(a1), expected_moves);
}

#[test]
fn king_can_capture_enemy_piece() {
    let mut board = Board::empty();
    let e4 = Square::new(4, 3).unwrap();
    let e5 = Square::new(4, 4).unwrap();

    board.set_piece(e4, Piece::new(Color::White, PieceKind::King));
    board.set_piece(e5, Piece::new(Color::Black, PieceKind::Pawn));

    assert!(board.king_moves_from(e4).contains(&ChessMove::new(e4, e5)));
}

#[test]
fn king_cannot_capture_own_piece() {
    let mut board = Board::empty();
    let e4 = Square::new(4, 3).unwrap();
    let e5 = Square::new(4, 4).unwrap();

    board.set_piece(e4, Piece::new(Color::White, PieceKind::King));
    board.set_piece(e5, Piece::new(Color::White, PieceKind::Pawn));

    assert!(!board.king_moves_from(e4).contains(&ChessMove::new(e4, e5)));
}

#[test]
fn empty_square_has_no_king_moves() {
    let board = Board::empty();
    let e4 = Square::new(4, 3).unwrap();

    assert!(board.king_moves_from(e4).is_empty());
}

#[test]
fn knight_square_has_no_king_moves() {
    let mut board = Board::empty();
    let e4 = Square::new(4, 3).unwrap();

    board.set_piece(e4, Piece::new(Color::White, PieceKind::Knight));

    assert!(board.king_moves_from(e4).is_empty());
}

#[test]
fn rook_on_e4_can_move_along_rank_and_file() {
    let mut board = Board::empty();
    let e4 = Square::new(4, 3).unwrap();

    board.set_piece(e4, Piece::new(Color::White, PieceKind::Rook));

    let expected_moves = vec![
        ChessMove::new(e4, Square::new(4, 4).unwrap()),
        ChessMove::new(e4, Square::new(4, 5).unwrap()),
        ChessMove::new(e4, Square::new(4, 6).unwrap()),
        ChessMove::new(e4, Square::new(4, 7).unwrap()),
        ChessMove::new(e4, Square::new(5, 3).unwrap()),
        ChessMove::new(e4, Square::new(6, 3).unwrap()),
        ChessMove::new(e4, Square::new(7, 3).unwrap()),
        ChessMove::new(e4, Square::new(4, 2).unwrap()),
        ChessMove::new(e4, Square::new(4, 1).unwrap()),
        ChessMove::new(e4, Square::new(4, 0).unwrap()),
        ChessMove::new(e4, Square::new(3, 3).unwrap()),
        ChessMove::new(e4, Square::new(2, 3).unwrap()),
        ChessMove::new(e4, Square::new(1, 3).unwrap()),
        ChessMove::new(e4, Square::new(0, 3).unwrap()),
    ];

    assert_eq!(board.rook_moves_from(e4), expected_moves);
}

#[test]
fn rook_stops_before_own_piece() {
    let mut board = Board::empty();
    let e4 = Square::new(4, 3).unwrap();
    let e5 = Square::new(4, 4).unwrap();
    let e6 = Square::new(4, 5).unwrap();
    let e7 = Square::new(4, 6).unwrap();

    board.set_piece(e4, Piece::new(Color::White, PieceKind::Rook));
    board.set_piece(e6, Piece::new(Color::White, PieceKind::Pawn));

    let moves = board.rook_moves_from(e4);

    assert!(moves.contains(&ChessMove::new(e4, e5)));
    assert!(!moves.contains(&ChessMove::new(e4, e6)));
    assert!(!moves.contains(&ChessMove::new(e4, e7)));
}

#[test]
fn rook_stops_before_own_piece_on_same_rank() {
    let mut board = Board::empty();
    let e4 = Square::new(4, 3).unwrap();
    let f4 = Square::new(5, 3).unwrap();
    let g4 = Square::new(6, 3).unwrap();
    let h4 = Square::new(7, 3).unwrap();

    board.set_piece(e4, Piece::new(Color::White, PieceKind::Rook));
    board.set_piece(g4, Piece::new(Color::White, PieceKind::Pawn));

    let moves = board.rook_moves_from(e4);

    assert!(moves.contains(&ChessMove::new(e4, f4)));
    assert!(!moves.contains(&ChessMove::new(e4, g4)));
    assert!(!moves.contains(&ChessMove::new(e4, h4)));
}

#[test]
fn rook_can_capture_enemy_piece_then_stops() {
    let mut board = Board::empty();
    let e4 = Square::new(4, 3).unwrap();
    let e5 = Square::new(4, 4).unwrap();
    let e6 = Square::new(4, 5).unwrap();
    let e7 = Square::new(4, 6).unwrap();

    board.set_piece(e4, Piece::new(Color::White, PieceKind::Rook));
    board.set_piece(e6, Piece::new(Color::Black, PieceKind::Pawn));

    let moves = board.rook_moves_from(e4);

    assert!(moves.contains(&ChessMove::new(e4, e5)));
    assert!(moves.contains(&ChessMove::new(e4, e6)));
    assert!(!moves.contains(&ChessMove::new(e4, e7)));
}

#[test]
fn rook_on_a1_does_not_wrap() {
    let mut board = Board::empty();
    let a1 = Square::new(0, 0).unwrap();

    board.set_piece(a1, Piece::new(Color::White, PieceKind::Rook));

    let expected_moves = vec![
        ChessMove::new(a1, Square::new(0, 1).unwrap()),
        ChessMove::new(a1, Square::new(0, 2).unwrap()),
        ChessMove::new(a1, Square::new(0, 3).unwrap()),
        ChessMove::new(a1, Square::new(0, 4).unwrap()),
        ChessMove::new(a1, Square::new(0, 5).unwrap()),
        ChessMove::new(a1, Square::new(0, 6).unwrap()),
        ChessMove::new(a1, Square::new(0, 7).unwrap()),
        ChessMove::new(a1, Square::new(1, 0).unwrap()),
        ChessMove::new(a1, Square::new(2, 0).unwrap()),
        ChessMove::new(a1, Square::new(3, 0).unwrap()),
        ChessMove::new(a1, Square::new(4, 0).unwrap()),
        ChessMove::new(a1, Square::new(5, 0).unwrap()),
        ChessMove::new(a1, Square::new(6, 0).unwrap()),
        ChessMove::new(a1, Square::new(7, 0).unwrap()),
    ];

    assert_eq!(board.rook_moves_from(a1), expected_moves);
}

#[test]
fn empty_square_has_no_rook_moves() {
    let board = Board::empty();
    let e4 = Square::new(4, 3).unwrap();

    assert!(board.rook_moves_from(e4).is_empty());
}

#[test]
fn king_square_has_no_rook_moves() {
    let mut board = Board::empty();
    let e4 = Square::new(4, 3).unwrap();

    board.set_piece(e4, Piece::new(Color::White, PieceKind::King));

    assert!(board.rook_moves_from(e4).is_empty());
}

#[test]
fn bishop_on_e4_can_move_diagonally() {
    let mut board = Board::empty();
    let e4 = Square::new(4, 3).unwrap();

    board.set_piece(e4, Piece::new(Color::White, PieceKind::Bishop));

    let expected_moves = vec![
        ChessMove::new(e4, Square::new(5, 4).unwrap()),
        ChessMove::new(e4, Square::new(6, 5).unwrap()),
        ChessMove::new(e4, Square::new(7, 6).unwrap()),
        ChessMove::new(e4, Square::new(3, 4).unwrap()),
        ChessMove::new(e4, Square::new(2, 5).unwrap()),
        ChessMove::new(e4, Square::new(1, 6).unwrap()),
        ChessMove::new(e4, Square::new(0, 7).unwrap()),
        ChessMove::new(e4, Square::new(5, 2).unwrap()),
        ChessMove::new(e4, Square::new(6, 1).unwrap()),
        ChessMove::new(e4, Square::new(7, 0).unwrap()),
        ChessMove::new(e4, Square::new(3, 2).unwrap()),
        ChessMove::new(e4, Square::new(2, 1).unwrap()),
        ChessMove::new(e4, Square::new(1, 0).unwrap()),
    ];

    assert_eq!(board.bishop_moves_from(e4), expected_moves);
}

#[test]
fn bishop_stops_before_own_piece() {
    let mut board = Board::empty();
    let e4 = Square::new(4, 3).unwrap();
    let f5 = Square::new(5, 4).unwrap();
    let g6 = Square::new(6, 5).unwrap();
    let h7 = Square::new(7, 6).unwrap();

    board.set_piece(e4, Piece::new(Color::White, PieceKind::Bishop));
    board.set_piece(g6, Piece::new(Color::White, PieceKind::Pawn));

    let moves = board.bishop_moves_from(e4);

    assert!(moves.contains(&ChessMove::new(e4, f5)));
    assert!(!moves.contains(&ChessMove::new(e4, g6)));
    assert!(!moves.contains(&ChessMove::new(e4, h7)));
}

#[test]
fn bishop_can_capture_enemy_piece_then_stops() {
    let mut board = Board::empty();
    let e4 = Square::new(4, 3).unwrap();
    let f5 = Square::new(5, 4).unwrap();
    let g6 = Square::new(6, 5).unwrap();
    let h7 = Square::new(7, 6).unwrap();

    board.set_piece(e4, Piece::new(Color::White, PieceKind::Bishop));
    board.set_piece(g6, Piece::new(Color::Black, PieceKind::Pawn));

    let moves = board.bishop_moves_from(e4);

    assert!(moves.contains(&ChessMove::new(e4, f5)));
    assert!(moves.contains(&ChessMove::new(e4, g6)));
    assert!(!moves.contains(&ChessMove::new(e4, h7)));
}

#[test]
fn bishop_on_a1_does_not_wrap() {
    let mut board = Board::empty();
    let a1 = Square::new(0, 0).unwrap();

    board.set_piece(a1, Piece::new(Color::White, PieceKind::Bishop));

    let expected_moves = vec![
        ChessMove::new(a1, Square::new(1, 1).unwrap()),
        ChessMove::new(a1, Square::new(2, 2).unwrap()),
        ChessMove::new(a1, Square::new(3, 3).unwrap()),
        ChessMove::new(a1, Square::new(4, 4).unwrap()),
        ChessMove::new(a1, Square::new(5, 5).unwrap()),
        ChessMove::new(a1, Square::new(6, 6).unwrap()),
        ChessMove::new(a1, Square::new(7, 7).unwrap()),
    ];

    assert_eq!(board.bishop_moves_from(a1), expected_moves);
}

#[test]
fn empty_square_has_no_bishop_moves() {
    let board = Board::empty();
    let e4 = Square::new(4, 3).unwrap();

    assert!(board.bishop_moves_from(e4).is_empty());
}

#[test]
fn rook_square_has_no_bishop_moves() {
    let mut board = Board::empty();
    let e4 = Square::new(4, 3).unwrap();

    board.set_piece(e4, Piece::new(Color::White, PieceKind::Rook));

    assert!(board.bishop_moves_from(e4).is_empty());
}

#[test]
fn queen_on_e4_can_move_along_rank_file_and_diagonal() {
    let mut board = Board::empty();
    let e4 = Square::new(4, 3).unwrap();

    board.set_piece(e4, Piece::new(Color::White, PieceKind::Queen));

    let expected_moves = vec![
        ChessMove::new(e4, Square::new(4, 4).unwrap()),
        ChessMove::new(e4, Square::new(4, 5).unwrap()),
        ChessMove::new(e4, Square::new(4, 6).unwrap()),
        ChessMove::new(e4, Square::new(4, 7).unwrap()),
        ChessMove::new(e4, Square::new(5, 3).unwrap()),
        ChessMove::new(e4, Square::new(6, 3).unwrap()),
        ChessMove::new(e4, Square::new(7, 3).unwrap()),
        ChessMove::new(e4, Square::new(4, 2).unwrap()),
        ChessMove::new(e4, Square::new(4, 1).unwrap()),
        ChessMove::new(e4, Square::new(4, 0).unwrap()),
        ChessMove::new(e4, Square::new(3, 3).unwrap()),
        ChessMove::new(e4, Square::new(2, 3).unwrap()),
        ChessMove::new(e4, Square::new(1, 3).unwrap()),
        ChessMove::new(e4, Square::new(0, 3).unwrap()),
        ChessMove::new(e4, Square::new(5, 4).unwrap()),
        ChessMove::new(e4, Square::new(6, 5).unwrap()),
        ChessMove::new(e4, Square::new(7, 6).unwrap()),
        ChessMove::new(e4, Square::new(3, 4).unwrap()),
        ChessMove::new(e4, Square::new(2, 5).unwrap()),
        ChessMove::new(e4, Square::new(1, 6).unwrap()),
        ChessMove::new(e4, Square::new(0, 7).unwrap()),
        ChessMove::new(e4, Square::new(5, 2).unwrap()),
        ChessMove::new(e4, Square::new(6, 1).unwrap()),
        ChessMove::new(e4, Square::new(7, 0).unwrap()),
        ChessMove::new(e4, Square::new(3, 2).unwrap()),
        ChessMove::new(e4, Square::new(2, 1).unwrap()),
        ChessMove::new(e4, Square::new(1, 0).unwrap()),
    ];

    let moves = board.queen_moves_from(e4);

    assert_eq!(moves.len(), expected_moves.len());
    for expected_move in expected_moves {
        assert!(moves.contains(&expected_move));
    }
}

#[test]
fn queen_stops_before_own_piece_on_file() {
    let mut board = Board::empty();
    let e4 = Square::new(4, 3).unwrap();
    let e5 = Square::new(4, 4).unwrap();
    let e6 = Square::new(4, 5).unwrap();
    let e7 = Square::new(4, 6).unwrap();

    board.set_piece(e4, Piece::new(Color::White, PieceKind::Queen));
    board.set_piece(e6, Piece::new(Color::White, PieceKind::Pawn));

    let moves = board.queen_moves_from(e4);

    assert!(moves.contains(&ChessMove::new(e4, e5)));
    assert!(!moves.contains(&ChessMove::new(e4, e6)));
    assert!(!moves.contains(&ChessMove::new(e4, e7)));
}

#[test]
fn queen_can_capture_enemy_piece_then_stops_on_diagonal() {
    let mut board = Board::empty();
    let e4 = Square::new(4, 3).unwrap();
    let f5 = Square::new(5, 4).unwrap();
    let g6 = Square::new(6, 5).unwrap();
    let h7 = Square::new(7, 6).unwrap();

    board.set_piece(e4, Piece::new(Color::White, PieceKind::Queen));
    board.set_piece(g6, Piece::new(Color::Black, PieceKind::Pawn));

    let moves = board.queen_moves_from(e4);

    assert!(moves.contains(&ChessMove::new(e4, f5)));
    assert!(moves.contains(&ChessMove::new(e4, g6)));
    assert!(!moves.contains(&ChessMove::new(e4, h7)));
}

#[test]
fn queen_on_a1_does_not_wrap() {
    let mut board = Board::empty();
    let a1 = Square::new(0, 0).unwrap();

    board.set_piece(a1, Piece::new(Color::White, PieceKind::Queen));

    let expected_moves = vec![
        ChessMove::new(a1, Square::new(0, 1).unwrap()),
        ChessMove::new(a1, Square::new(0, 2).unwrap()),
        ChessMove::new(a1, Square::new(0, 3).unwrap()),
        ChessMove::new(a1, Square::new(0, 4).unwrap()),
        ChessMove::new(a1, Square::new(0, 5).unwrap()),
        ChessMove::new(a1, Square::new(0, 6).unwrap()),
        ChessMove::new(a1, Square::new(0, 7).unwrap()),
        ChessMove::new(a1, Square::new(1, 0).unwrap()),
        ChessMove::new(a1, Square::new(2, 0).unwrap()),
        ChessMove::new(a1, Square::new(3, 0).unwrap()),
        ChessMove::new(a1, Square::new(4, 0).unwrap()),
        ChessMove::new(a1, Square::new(5, 0).unwrap()),
        ChessMove::new(a1, Square::new(6, 0).unwrap()),
        ChessMove::new(a1, Square::new(7, 0).unwrap()),
        ChessMove::new(a1, Square::new(1, 1).unwrap()),
        ChessMove::new(a1, Square::new(2, 2).unwrap()),
        ChessMove::new(a1, Square::new(3, 3).unwrap()),
        ChessMove::new(a1, Square::new(4, 4).unwrap()),
        ChessMove::new(a1, Square::new(5, 5).unwrap()),
        ChessMove::new(a1, Square::new(6, 6).unwrap()),
        ChessMove::new(a1, Square::new(7, 7).unwrap()),
    ];

    let moves = board.queen_moves_from(a1);

    assert_eq!(moves.len(), expected_moves.len());
    for expected_move in expected_moves {
        assert!(moves.contains(&expected_move));
    }
}

#[test]
fn empty_square_has_no_queen_moves() {
    let board = Board::empty();
    let e4 = Square::new(4, 3).unwrap();

    assert!(board.queen_moves_from(e4).is_empty());
}

#[test]
fn bishop_square_has_no_queen_moves() {
    let mut board = Board::empty();
    let e4 = Square::new(4, 3).unwrap();

    board.set_piece(e4, Piece::new(Color::White, PieceKind::Bishop));

    assert!(board.queen_moves_from(e4).is_empty());
}
