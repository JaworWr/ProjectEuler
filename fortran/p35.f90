module p35_funcs
    implicit none
    
contains
    pure function is_prime(x)
        integer, intent(in) :: x
        logical :: is_prime
        integer :: d

        is_prime = .true.

        if (x < 2) then
            is_prime = .false.
            return
        end if

        d = 2
        do while (d * d <= x)
            if (modulo(x, d) == 0) then
                is_prime = .false.
                return
            end if
            d = d + 1
        end do

    end function is_prime

    pure function rotate(x) result(r)
        integer, intent(in) :: x
        integer :: r, d

        d = floor(log10(float(x)))
        r = x / 10 + 10 ** d * modulo(x, 10)
    end function rotate

    elemental function check(x) result(ok)
        integer, intent(in) :: x
        integer :: cur
        logical :: ok

        cur = x
        ok = .true.
        if (.not. is_prime(x)) then
            ok = .false.
            return
        end if
        cur = rotate(cur)

        do while (cur /= x)
            if (.not. is_prime(cur)) then
                ok = .false.
                return
            end if
            cur = rotate(cur)
        end do

    end function check
end module p35_funcs

program p35
    use p35_funcs
    implicit none
    integer, parameter :: MAX = 1000000
    integer :: i

    print *, count(check([(i, i = 1, MAX - 1)]))
end program p35
