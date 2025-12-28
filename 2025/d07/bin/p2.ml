let rec every_other = function e :: _ :: r -> e :: every_other r | _ -> []

let read_input filename =
  In_channel.with_open_text filename In_channel.input_lines |> every_other

let equal_string ch s =
  List.init (String.length s) (fun i -> if String.get s i = ch then 1 else 0)

let get_dir i beams splitters on_splitter =
  let b = List.nth beams i in
  let s = List.nth splitters i in
  if s = on_splitter then b else 0

let split_beam beams splitters i =
  let from_l = if i > 0 then get_dir (i - 1) beams splitters 1 else 0 in
  let from_t = get_dir i beams splitters 0 in
  let from_r =
    if i + 1 < List.length beams then get_dir (i + 1) beams splitters 1 else 0
  in
  from_r + from_t + from_l

let drop beams splitters =
  let vals = List.init (List.length beams) (split_beam beams splitters) in
  vals

let () =
  match read_input "input.txt" with
  | start :: splitters ->
      let init = equal_string 'S' start in
      List.map (equal_string '^') splitters
      |> List.fold_left drop init |> List.fold_left ( + ) 0
      |> Printf.printf "%d\n"
  | _ -> invalid_arg "invalid input"
