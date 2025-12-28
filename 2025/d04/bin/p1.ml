let read_input filename =
  In_channel.with_open_text filename In_channel.input_lines
  |> List.map (fun str -> Array.init (String.length str) (String.get str))
  |> Array.of_list

let get x y map =
  if x < 0 || y < 0 || y >= Array.length map || x >= Array.length map.(0) then
    None
  else Some map.(y).(x)

let can_be_picked_up x y map =
  if get x y map = Some '.' then 0
  else
    let neighbors =
      [ (0, 1); (0, -1); (1, 0); (1, 1); (1, -1); (-1, 0); (-1, 1); (-1, -1) ]
      |> List.map (fun (xo, yo) -> get (x + xo) (y + yo) map)
      |> List.fold_left
           (fun acc ch -> match ch with Some '@' -> acc + 1 | _ -> acc)
           0
    in
    if neighbors < 4 then 1 else 0

let rec solve y acc map =
  let rec solve_col x y acc map =
    if x >= Array.length map.(0) then acc
    else solve_col (x + 1) y (acc + can_be_picked_up x y map) map
  in
  if y >= Array.length map then acc
  else solve (y + 1) (acc + solve_col 0 y 0 map) map

let () = read_input "input.txt" |> solve 0 0 |> Printf.printf "%d\n"
