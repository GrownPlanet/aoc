let read_input filename =
  In_channel.with_open_text filename In_channel.input_lines
  |> List.map (fun str -> Array.init (String.length str) (String.get str))
  |> Array.of_list

let count_at map =
  Array.fold_left
    (fun acc a ->
      acc
      + Array.fold_left (fun acc ch -> if ch = '@' then acc + 1 else acc) 0 a)
    0 map

let get x y map =
  if x < 0 || y < 0 || y >= Array.length map || x >= Array.length map.(0) then
    None
  else Some map.(y).(x)

let can_be_picked_up x y map =
  if get x y map = Some '.' then true
  else
    let neighbors =
      [ (0, 1); (0, -1); (1, 0); (1, 1); (1, -1); (-1, 0); (-1, 1); (-1, -1) ]
      |> List.map (fun (xo, yo) -> get (x + xo) (y + yo) map)
      |> List.fold_left
           (fun acc ch -> match ch with Some '@' -> acc + 1 | _ -> acc)
           0
    in
    if neighbors < 4 then true else false

let update_map map =
  Array.mapi
    (fun y row ->
      Array.mapi (fun x _ -> if can_be_picked_up x y map then '.' else '@') row)
    map

let rec solve map =
  let new_map = update_map map in
  if map = new_map then count_at new_map else solve new_map

let () =
  let map = read_input "input.txt" in
  let initial_count = count_at map in
  let reduced_count = solve map in
  Printf.printf "%d\n" (initial_count - reduced_count)
