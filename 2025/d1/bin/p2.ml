let read_file filename =
  In_channel.with_open_text filename In_channel.input_lines

type rotation = { dir : char; amount : int }

let parse_line line =
  Scanf.sscanf line "%[LR]%d" (fun dir amount ->
      { dir = String.get dir 0; amount })

let put_in_range num =
  let wnum = num mod 100 in
  if num > 99 then
    (wnum, num / 100)
  else if num < 0 then
    ((100 + wnum) mod 100, (-num / 100) + 1 - ((100 + wnum) / 100))
  else (num, 0)

let rotate position dir amount =
  let new_position, over0 =
    (match dir with
      | 'L' -> position - amount
      | 'R' -> position + amount
      | _ -> invalid_arg "not left or right")
    |> put_in_range
  in
  let over0 = if position = 0 && dir = 'L' then over0 - 1 else over0 in
  let over0 = if new_position = 0 && dir = 'L' then over0 + 1 else over0 in
  (new_position, over0)

let rec solve position list =
  match list with
  | [] -> if position = 0 then 1 else 0
  | r :: t ->
      let new_position, over0count = rotate position r.dir r.amount in
      let is0 = if new_position = 0 then 1 else 0 in
      over0count + (is0 * 0) + solve new_position t

let () =
  read_file "input.txt" |> List.map parse_line |> solve 50
  |> Printf.printf "%d\n"
