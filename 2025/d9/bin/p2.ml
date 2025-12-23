let read_input filename =
  In_channel.with_open_text filename In_channel.input_lines

let parse line = Scanf.sscanf line "%d,%d" (fun x y -> (x, y))
let get_area (x1, y1) (x2, y2) = (abs (x2 - x1) + 1) * (abs (y2 - y1) + 1)
let inbi v s e = min s e <= v && v < max s e
let inbx v s e = min s e < v && v < max s e

let rec count_intersections acc s polygon =
  let intersects (ax, ay) (cx, cy) (dx, dy) =
    (inbi ax cx dx && ay <= cy) || (inbi ay cy dy && cx = ax)
  in
  match polygon with
  | p1 :: p2 :: r ->
      if intersects s p1 p2 then count_intersections (not acc) s (p2 :: r)
      else count_intersections acc s (p2 :: r)
  | _ -> acc

let rec inters_line s e polygon =
  let intersects (ax, ay) (bx, by) (cx, cy) (dx, dy) =
    match (ax = bx, cx = dx) with
    | true, true | false, false -> false
    | true, false -> inbx ax cx dx && inbx cy ay by
    | false, true -> inbx cx ax bx && inbx ay cy dy
  in
  match polygon with
  | p1 :: p2 :: r ->
      if intersects s e p1 p2 then true else inters_line s e (p2 :: r)
  | _ -> false

let in_polygon = count_intersections true

let rec get_areas_with acc p list polygon =
  match list with
  | p2 :: r ->
      let x1, y1 = p in
      let x2, y2 = p2 in
      if
        inters_line (x1, y1) (x1, y2) polygon
        || inters_line (x1, y1) (x2, y1) polygon
        || inters_line (x2, y2) (x1, y2) polygon
        || inters_line (x2, y2) (x2, y1) polygon
        || in_polygon (x1, y2) polygon 
        || in_polygon (x2, y1) polygon
      then get_areas_with acc p r polygon
      else get_areas_with (max acc (get_area p p2)) p r polygon
  | [] -> acc

let rec get_areas acc list polygon =
  match list with
  | p :: r -> get_areas (max acc (get_areas_with 0 p r polygon)) r polygon
  | [] -> acc

let rec fix_polygon polygon =
  match polygon with
  | (sx, sy) :: (ex, ey) :: r ->
      if ex > sx then (sx, sy - 1) :: fix_polygon ((ex, ey - 1) :: r)
      else if ey > sy then (sx + 1, sy) :: fix_polygon ((ex + 1, ey) :: r)
      else (sx, sy) :: fix_polygon ((ex, ey) :: r)
  | e -> e

let () =
  let locations = read_input "input.txt" |> List.map parse in
  let polygon = locations |> fix_polygon in
  get_areas 0 locations polygon |> Printf.printf "%d\n"
