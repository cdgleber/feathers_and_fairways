UPDATE hole_scores
SET fantasy_points = CASE
    WHEN score_to_par <= -3 THEN 8
    WHEN score_to_par = -2 THEN 5
    WHEN score_to_par = -1 THEN 2
    WHEN score_to_par = 0 THEN 1
    WHEN score_to_par = 1 THEN
        CASE WHEN golfer_id IN (SELECT id FROM golfers WHERE is_amateur = 1) THEN 0 ELSE -1 END
    ELSE
        CASE WHEN golfer_id IN (SELECT id FROM golfers WHERE is_amateur = 1) THEN 0 ELSE -3 END
END;
