module p92_funcs

    implicit none
    public

    integer, parameter :: MAX = 10000000
    integer, dimension(MAX) :: e = 0

contains
    elemental function sods(x)
        integer, value :: x
        integer :: sods, d

        sods = 0
        do while (x .gt. 0)
            d = modulo(x, 10)
            sods = sods + d ** 2
            x = x / 10
        end do
    end

    recursive subroutine run_cycle(x)
        integer, value :: x
        integer :: x1

        if (e(x) .ne. 0) then
            return
        end if

        x1 = sods(x)
        call run_cycle(x1)
        e(x) = e(x1)
    end

end module p92_funcs

program p92

    use p92_funcs
    implicit none

    integer :: i
    
    e(1) = 1
    e(89) = 89

    do i = 1, MAX
        call run_cycle(i)
    end do
    print *, sum(ltoi([(e(i) .eq. 89, i = 1, MAX)]))

contains
    elemental function ltoi(x) result(n)
        logical, intent(in) :: x
        integer :: n
        if (x) then
            n = 1
        else
            n = 0
        end if
    end
end program p92
