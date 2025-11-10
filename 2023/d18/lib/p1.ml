let read_input filename =
  In_channel.with_open_text filename In_channel.input_lines

let parse line =
  let split = String.split_on_char ' ' line in
  match split with
  | [ dir; len; _ ] -> (dir, int_of_string len)
  | _ -> invalid_arg "invalid line"

let sign num = if num < 0 then -1 else 1

let calculate_area lines =
  let rec helper lines area_acc line_acc x y =
    match lines with
    | [] -> abs area_acc + (line_acc / 2) + 1
    | line :: rest ->
        let dir, len = parse line in
        let new_x, new_y =
          match dir with
          | "U" -> (x, y - len)
          | "D" -> (x, y + len)
          | "L" -> (x - len, y)
          | "R" -> (x + len, y)
          | _ -> invalid_arg "invalid direction"
        in
        let area = (new_x - x) * y in
        helper rest (area_acc + area) (line_acc + len) new_x new_y
  in
  helper lines 0 0 0 0

let solve () = read_input "input.txt" |> calculate_area |> Printf.printf "%d\n"
