let rec every_other = function e :: _ :: r -> e :: every_other r | _ -> []

let read_input filename =
  In_channel.with_open_text filename In_channel.input_lines |> every_other

let equal_string ch s =
  List.init (String.length s) (fun i -> if String.get s i = ch then 1 else 0)

let rec count_overlap acc l1 l2 =
  match (l1, l2) with
  | a :: r1, b :: r2 ->
      let overlapping = if a = 1 && b = 1 then 1 else 0 in
      count_overlap (acc + overlapping) r1 r2
  | _ -> acc

let get_dir i beams splitters on_splitter =
  let b = List.nth beams i in
  let s = List.nth splitters i in
  if s = on_splitter && b = 1 then 1 else 0

let split_beam beams splitters i =
  if get_dir i beams splitters 0 = 1 then 1
  else if i > 0 && get_dir (i - 1) beams splitters 1 = 1 then 1
  else if i + 1 < List.length beams then get_dir (i + 1) beams splitters 1
  else 0

let drop (beams, acc) splitters =
  let vals = List.init (List.length beams) (split_beam beams splitters) in
  let overlap = count_overlap 0 beams splitters in
  (vals, acc + overlap)

let () =
  match read_input "input.txt" with
  | start :: splitters ->
      let init = equal_string 'S' start in
      let _, res =
        List.map (equal_string '^') splitters |> List.fold_left drop (init, 0)
      in
      Printf.printf "%d\n" res
  | _ -> invalid_arg "invalid input"
