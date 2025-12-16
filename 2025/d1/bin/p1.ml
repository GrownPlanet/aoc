let read_file filename =
  In_channel.with_open_text filename In_channel.input_lines

type rotation = { dir : char; count : int }

let parse_line line =
  Scanf.sscanf line "%[LR]%d" (fun dir count ->
      { dir = String.get dir 0; count })

let put_in_range num =
  let v = num mod 100 in
  if v < 0 then 100 + v else v

let rotate position dir count =
  let new_position =
    match dir with
    | 'L' -> position - count
    | 'R' -> position + count
    | _ -> invalid_arg "not left or right"
  in
  put_in_range new_position

let rec solve position list =
  match list with
  | [] -> if position = 0 then 1 else 0
  | r :: t ->
      let new_position = rotate position r.dir r.count in
      let v = if new_position = 0 then 1 else 0 in
      v + solve new_position t

let () =
  read_file "input.txt" |> List.map parse_line |> solve 50
  |> Printf.printf "%d\n"
