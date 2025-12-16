let read_file filename =
  In_channel.with_open_text filename In_channel.input_lines

type rotation = { dir : char; count : int }

let parse_line line =
  Scanf.sscanf line "%[LR]%d" (fun dir count ->
      { dir = String.get dir 0; count })

let normalize num =
  let v = num mod 100 in
  if v < 0 then 100 + v else v

let rotate position dir count =
  let new_position =
    match dir with
    | 'L' -> position - count
    | 'R' -> position + count
    | _ -> invalid_arg "neither left nor right"
  in
  normalize new_position

let solve (position, acc) offset =
  let new_position = rotate position offset.dir offset.count in
  let is_zero = if new_position = 0 then 1 else 0 in
  (new_position, acc + is_zero)

let () =
  let _, res =
    read_file "input.txt" |> List.map parse_line |> List.fold_left solve (50, 0)
  in
  Printf.printf "%d\n" res
