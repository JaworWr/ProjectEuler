module p36_procs
    implicit none
    
contains
    elemental function is_palindromic(x, b)
        integer, value :: x, b
        logical :: is_palindromic
        integer, dimension(32), target :: digit_buffer
        integer, dimension(:), pointer :: digits
        integer :: i, l

        is_palindromic = .true.
        i = 1
        do while (x > 0)
            digit_buffer(i) = mod(x, b)
            x = x / b
            i = i + 1
        end do
        l = i - 1
        digits => digit_buffer(1:l)

        do i = 1, l / 2
            if (digit_buffer(i) /= digit_buffer(l - i + 1)) then
                is_palindromic = .false.
                return
            end if
        end do

    end function is_palindromic
    
end module p36_procs

program p36
    use p36_procs
    implicit none

    integer, parameter :: MAX = 1000000 - 1
    integer, dimension(:), allocatable :: inputs
    logical, dimension(:), allocatable :: m
    integer :: i, result

    inputs = [(i, i = 1, MAX)]
    m = is_palindromic(inputs, 10) .and. is_palindromic(inputs, 2)
    result = sum(pack(inputs, m))
    print *, result

end program p36
