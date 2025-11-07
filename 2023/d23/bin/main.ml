let read_input filename =
  let ic = In_channel.open_text filename in
  let lines = In_channel.input_lines ic in
  List.map
    (fun l -> Array.init (String.length l) (fun i -> String.get l i))
    lines
  |> Array.of_list

let in_bounds map (x, y) =
  x >= 0 && y >= 0 && x < Array.length map.(0) && y < Array.length map

let possible_path (ox, oy) (x, y) map =
  let ch = map.(y).(x) in
  match (ch, abs ox, abs oy) with
  | '.', _, _ -> true
  | '>', 1, 0 -> true
  | '<', 1, 0 -> true
  | 'v', 0, 1 -> true
  | '^', 0, 1 -> true
  | _ -> false

let get_neighbors (x, y) map =
  [ (-1, 0); (1, 0); (0, -1); (0, 1) ]
  |> List.map (fun (ox, oy) -> ((ox, oy), (x + ox, y + oy)))
  |> List.filter (fun (off, loc) ->
      in_bounds map loc && possible_path off loc map)
  |> List.map (fun (_, loc) -> loc)

let get_longest_path startl endl map =
  let rec dfs visited current =
      if current = endl then
        List.length visited
      else
        get_neighbors current map
          |> List.filter (fun n -> not (List.mem n visited))
          |> List.map (fun n -> dfs (current :: visited) n)
          |> List.fold_left (fun acc l -> max acc l) 0
  in
  dfs [] startl

let () =
  let map = read_input "input.txt" in
  let start_loc = (1, 0) in
  let end_loc = (Array.length map.(0) - 2, Array.length map - 1) in
  get_longest_path start_loc end_loc map |> print_int;
  print_newline ();
