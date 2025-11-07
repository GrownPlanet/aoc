let read_file filename =
  let ic = In_channel.open_text filename in
  let lines = In_channel.input_lines ic in
  In_channel.close ic;
  lines

let split_input lines =
  let rec helper acc rem =
    match rem with
    | s :: rest ->
        if String.length s = 0 then (List.rev (List.tl acc), rest)
        else helper (s :: acc) rest
    | [] -> (acc, [])
  in
  helper [] lines

let rec transpose list =
  match list with
  | [] -> []
  | [] :: r -> transpose r
  | r -> List.(map hd r :: transpose (map tl r))

let parse_crates crates =
  let crate_count = (String.length (List.nth crates 0) + 1) / 4 in
  crates
  |> List.map (fun c ->
      List.init crate_count (fun i -> String.get c ((4 * i) + 1)))
  |> transpose
  |> List.map (fun cs -> List.filter (fun c -> c <> ' ') cs)

type instruction = { count : int; from_idx : int; to_idx : int }

let parse_moves moves =
  List.map
    (fun move ->
      match String.split_on_char ' ' move with
      | [ _; count; _; from_idx; _; to_idx ] ->
          {
            count = int_of_string count;
            from_idx = int_of_string from_idx - 1;
            to_idx = int_of_string to_idx - 1;
          }
      | _ -> invalid_arg "expect '_ count _ from _ to' for a move")
    moves

let move_crates crates move =
  let rec split_list_at list acc at idx =
    match list with
    | [] -> ([], [])
    | h :: t ->
        if at = idx + 1 then (List.rev (h :: acc), t)
        else split_list_at t (h :: acc) at (idx + 1)
  in
  let front, back =
    split_list_at (List.nth crates move.from_idx) [] move.count 0
  in
  List.mapi
    (fun i l ->
      if i = move.from_idx then back
      (* Part 1 *)
      (* else if i = move.to_idx then (List.rev front) @ l *)
      (* Part 2 *)
      else if i = move.to_idx then front @ l
      else l)
    crates

let () =
  let filename = "input.txt" in
  let raw_crates, raw_moves = read_file filename |> split_input in
  let crates = parse_crates raw_crates in
  let moves = parse_moves raw_moves in
  let moved_crates = List.fold_left move_crates crates moves in
  List.iter (fun l -> print_char (List.hd l)) moved_crates;
  print_newline ()
